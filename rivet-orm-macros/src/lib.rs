extern crate proc_macro;

use proc_macro::TokenStream;
use proc_macro2::{Ident, TokenStream as TokenStream2};
use quote::quote;
use rivet_utils::inflection;
use syn::{Field, Fields, ItemStruct, LitStr, Meta, Type, meta, parse_macro_input, parse2};

struct ColumnMeta {
    ident: Ident,
    name: String,
    tp: Type,
}

#[proc_macro_attribute]
pub fn table(table_args: TokenStream, item: TokenStream) -> TokenStream {
    let struct_input = &mut parse_macro_input!(item as ItemStruct); // struct_input 包含了字段上的 #[col] 属性
    let table_name = parse_table_name(&struct_input.ident, table_args);

    // 只处理带名字的struct，例如: struct User{}
    // 其他的不处理，例如：struct Color(i32, i32, i32) 或者 struct Empty
    let mut column_metas: Vec<ColumnMeta> = vec![];
    if let Fields::Named(ref mut fields) = struct_input.fields {
        column_metas = fields
            .named
            .iter_mut()
            .filter_map(|field| {
                // 如果有 #[not_col]，从最终生成的结构体字段中移除该属性并跳过元数据生成
                if find_arg(field, "no_col", true).is_some() {
                    return None;
                }
                let col_token = find_arg(field, "col", true);

                // 获取字段在代码中的原始名称 (如 id)
                let field_ident = field.ident.as_ref()?.clone();
                let col_name = col_token
                    .and_then(|meta| parse_arg_value_from(meta, "name"))
                    .unwrap_or_else(|| inflection::snake_case_of(&field_ident.to_string()));

                // 关键点：这里我们要保留原始的字段标识符（或处理后的标识符）用于结构体成员
                Some(ColumnMeta { ident: field_ident, name: col_name, tp: field.ty.clone() })
            })
            .collect()
    }

    let metadata_gen = expand_columns_metadata(struct_input, column_metas, &table_name);

    // 3. 提取字段元数据

    // 生成代码：回填 struct 定义，并注入常量映射
    let expanded = quote! {
        #struct_input // 这里的 struct 已经过属性清理
        #metadata_gen
    };
    expanded.into()
}

fn get_column_type(ty: &Type) -> Type {
    match ty {
        // 1. 处理引用类型: &'a str -> String
        Type::Reference(type_ref) => {
            if let syn::Type::Path(ref tp) = *type_ref.elem {
                if tp.path.is_ident("str") {
                    // 直接构造 String 的 TypePath，避免复杂的 parse_quote 引用问题
                    return syn::parse_quote!(String);
                }
            }
            // 如果是其他引用（如 &i32），递归处理其内部
            get_column_type(&type_ref.elem)
        }
        // 2. 处理路径类型: Option<T> 或直接是 T
        Type::Path(tp) => {
            let last_segment = tp.path.segments.last().unwrap();
            // 检查是否是 Option
            if last_segment.ident == "Option" {
                if let syn::PathArguments::AngleBracketed(args) = &last_segment.arguments {
                    if let Some(syn::GenericArgument::Type(inner_ty)) = args.args.first() {
                        // 递归处理，防止 Option<Option<T>> 的情况
                        return get_column_type(inner_ty);
                    }
                }
            }
            ty.clone()
        }
        _ => ty.clone(),
    }
}
fn expand_columns_metadata(
    struct_input: &ItemStruct,
    metas: Vec<ColumnMeta>,
    table_name: &str,
) -> TokenStream2 {
    let column_consts = metas.iter().map(|m| {
        let field_ident = &m.ident;
        let column_name = &m.name;
        // --- 关键重构点 ---
        // 提取 Option<T> 中的 T，如果是 T 则保持不变
        let column_type = get_column_type(&m.tp);

        quote! {
            pub const #field_ident: ::rivet::orm::Column<#column_type> = ::rivet::orm::Column::new(#column_name);
        }
    });

    let struct_name = &struct_input.ident;
    quote! {
        #[allow(non_upper_case_globals)]
        impl #struct_name {
            pub const TABLE_NAME: &'static str = #table_name;
            #( #column_consts )*
        }
    }
}

fn find_arg(token: &mut Field, attr_name: &str, take: bool) -> Option<TokenStream2> {
    // 寻找属性位置
    let pos = token.attrs.iter().position(|a| a.path().is_ident(attr_name))?;

    let attr = if take {
        token.attrs.remove(pos) // 将属性从列表中移除
    } else {
        token.attrs[pos].clone()
    };

    // 转换并返回其内部 Tokens
    match attr.meta {
        Meta::List(l) => Some(l.tokens), // 匹配 #[col(name = "id")]
        Meta::Path(_) => Some(TokenStream2::new()), // 匹配 #[col] 或 #[no_col]
        Meta::NameValue(nv) => Some(quote!(#nv)), // 兼容 #[col = "id"]
    }
}

fn parse_table_name(struct_name: &Ident, table_args: TokenStream) -> String {
    // 解析表名：直接解析宏函数的第一个参数 table_args
    if table_args.is_empty() { None } else { parse_arg_value_from(table_args.into(), "name") }
        .unwrap_or_else(|| inflection::table_name_of(&struct_name.to_string()))
}

fn parse_arg_value_from(args: TokenStream2, arg_name: &str) -> Option<String> {
    if args.is_empty() {
        return None;
    }
    // #[table("users")] -> args 是 "users" (一个 LitStr)
    if let Ok(lit) = parse2::<LitStr>(args.clone()) {
        return Some(lit.value());
    }

    // #[table(users)] -> args 是 users (一个 Ident)
    if let Ok(ident) = parse2::<Ident>(args.clone()) {
        return Some(ident.to_string());
    }

    // #[table(name = "users")] 或 #[table(name = users)]
    // 使用 syn 2.0 提供的匿名解析器处理 arg_name = value
    let mut relt = None;
    let parser = meta::parser(|meta| {
        if meta.path.is_ident(arg_name) {
            let value = meta.value()?; // 得到 = 后面的内容
            if let Ok(lit) = value.parse::<LitStr>() {
                relt = Some(lit.value());
            } else if let Ok(ident) = value.parse::<Ident>() {
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
