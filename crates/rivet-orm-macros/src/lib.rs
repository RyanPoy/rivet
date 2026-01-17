extern crate proc_macro;

use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use rivet_utils::inflection;
use syn::{ItemStruct, Meta};

#[proc_macro_attribute]
pub fn table(table_args: TokenStream, item: TokenStream) -> TokenStream {
    // 1. 解析被修饰的 struct 整体
    // 注意：这里的 struct_input 包含了字段上的 #[col] 属性
    let struct_input: syn::ItemStruct = syn::parse_macro_input!(item as ItemStruct);
    let struct_name = &struct_input.ident;

    // 2. 解析表名：直接解析宏函数的第一个参数 table_args
    let table_name = if table_args.is_empty() {
        None
    } else {
        parse_arg_from(table_args.into(), "name")
    };
    let table_name = inflection::table_name_of(&table_name.unwrap_or(struct_name.to_string()));

    // 3. 提取字段元数据
    let mut columns = Vec::new();
    if let syn::Fields::Named(ref mut fields) = struct_input.fields {
        for field in &mut fields.named {
            // 获取字段在代码中的原始名称 (如 id)
            let field_ident = field.ident.as_ref().unwrap().to_string();

            // 查找该字段上是否挂了 #[col] 属性
            let col_attr = field.attrs.iter().find(|a| a.path().is_ident("col"));

            // 核心复用点：将 #[col(...)] 转换为与 table_args 相同的格式
            let col_name_attr = col_attr.and_then(|attr| {
                match &attr.meta {
                    // 如果是 #[col(name = "xxx")] 或 #[col("xxx")]
                    Meta::List(list) => parse_arg_from(list.tokens.clone(), "name"),
                    // 如果是单独的 #[col]
                    Meta::Path(_) => None,
                    _ => None,
                }
            });

            // 确定最终列名：手动指定 > 字段名
            let final_col_name = col_name_attr.unwrap_or(field_ident);
            columns.push(final_col_name);

            // 关键动作：清理掉字段上的 #[col] 属性
            // 否则生成的代码中保留 #[col] 会导致编译器报错（因为它不是标准属性）
            field.attrs.retain(|a| !a.path().is_ident("col"));
        }
    }

    let expanded = quote! {
        #struct_input

        impl #struct_name {
            pub const TABLE_NAME: &'static str = #table_name;
        }
    };
    expanded.into()
}

fn parse_arg_from(args: TokenStream2, key: &str) -> Option<String> {
    // #[table("users")] -> args 是 "users" (一个 LitStr)
    if let Ok(lit) = syn::parse2::<syn::LitStr>(args.clone()) {
        return Some(lit.value());
    }

    // #[table(users)] -> args 是 users (一个 Ident)
    if let Ok(ident) = syn::parse2::<syn::Ident>(args.clone()) {
        return Some(ident.to_string());
    }

    // #[table(name = "users")] 或 #[table(name = users)]
    // 使用 syn 2.0 提供的匿名解析器处理 key = value
    let mut relt = None;
    let parser = syn::meta::parser(|meta| {
        if meta.path.is_ident(key) {
            let value = meta.value()?; // 得到 = 后面的内容
            if let Ok(lit) = value.parse::<syn::LitStr>() {
                relt = Some(lit.value());
            } else if let Ok(ident) = value.parse::<syn::Ident>() {
                relt = Some(ident.to_string());
            }
        }
        Ok(())
    });

    // 尝试解析
    use syn::parse::Parser;
    let _ = parser.parse2(args);

    relt
}
