extern crate proc_macro;

use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use rivet_utils::inflection;
use syn::{Fields, ItemStruct, Meta, parse_macro_input};

#[proc_macro_attribute]
pub fn table(table_args: TokenStream, item: TokenStream) -> TokenStream {
    // 1. 解析被修饰的 struct 整体
    // 注意：这里的 struct_input 包含了字段上的 #[col] 属性
    let mut struct_input = parse_macro_input!(item as ItemStruct);
    let struct_name = &struct_input.ident;
    let vis = &struct_input.vis; // 保持可见性一致

    // 2. 解析表名：直接解析宏函数的第一个参数 table_args
    let table_name = if table_args.is_empty() {
        None
    } else {
        parse_arg_from(table_args.into(), "name")
    };
    let table_name =
        table_name.unwrap_or_else(|| inflection::table_name_of(&struct_name.to_string()));

    // 3. 提取字段元数据
    let mut column_idents = Vec::new(); // 用于 Columns 结构体的字段名
    let mut column_names = Vec::new(); // 对应的字符串值

    // 只处理带名字的struct，例如: struct User{}
    // 其他的不处理，例如：struct Color(i32, i32, i32) 或者 struct Empty
    if let Fields::Named(ref mut fields) = struct_input.fields {
        for field in &mut fields.named {
            // 获取字段在代码中的原始名称 (如 id)
            let field_ident = field.ident.as_ref().unwrap();

            // 查找该字段上是否挂了 #[col] 属性
            let col_attr = field.attrs.iter().find(|a| a.path().is_ident("col"));

            // 核心复用点：将 #[col(...)] 转换为与 table_args 相同的格式
            let col_name = col_attr.and_then(|attr| {
                match &attr.meta {
                    // 如果是 #[col(name = "xxx")] 或 #[col("xxx")]
                    Meta::List(list) => parse_arg_from(list.tokens.clone(), "name"),
                    // 如果是单独的 #[col]
                    Meta::Path(_) => None,
                    _ => None,
                }
            });

            // 确定最终列名：手动指定 > 字段名
            let col_name =
                col_name.unwrap_or_else(|| inflection::snake_case_of(&field_ident.to_string()));
            // columns.push(col_name);

            // 关键点：这里我们要保留原始的字段标识符（或处理后的标识符）用于结构体成员
            column_idents.push(field_ident.clone());
            column_names.push(col_name);

            // 关键动作：清理掉字段上的 #[col] 属性
            // 否则生成的代码中保留 #[col] 会导致编译器报错（因为它不是标准属性）
            field.attrs.retain(|a| !a.path().is_ident("col"));
        }
    }

    let columns_struct_name = quote::format_ident!("{}_Columns_Internal", struct_name);

    // 生成代码：回填 struct 定义，并注入常量映射
    let expanded = quote! {
        #struct_input // 这里的 struct 已经过属性清理

        #[allow(non_camel_case_types, non_upper_case_globals)]
        #vis struct #columns_struct_name;

        #[allow(non_upper_case_globals)]
        impl #columns_struct_name {
            #( pub const #column_idents: &'static str = #column_names; )*
        }

        impl #struct_name {
            pub const TABLE_NAME: &'static str = #table_name;

            // 这里是关键：定义一个关联常量，它的类型是上面的 struct
            // 这样 User::Columns::id 就能通过“常量实例”找到该类型的关联常量
            pub const Columns: #columns_struct_name = #columns_struct_name;
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
