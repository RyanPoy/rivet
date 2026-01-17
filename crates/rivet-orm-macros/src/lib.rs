extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use rivet_utils::inflection;
use syn::{Attribute, LitStr, Meta};

#[proc_macro_attribute]
pub fn table(attrs: TokenStream, item: TokenStream) -> TokenStream {
    // 解析 struct
    let struct_input: syn::ItemStruct =
        syn::parse(item).expect("`#[table]` can be applied to structs");
    let struct_name = &struct_input.ident;

    let table = struct_input
        .attrs
        .iter()
        .find(|attr| attr.path().is_ident("table"));
    let table_name = match table {
        // #[table]
        // #[table("users")]
        // #[table(users)]
        // #[table(name="users")]
        // #[table(name=users)]
        Some(attr) => parse_attr(attr, "name"),
        _ => None,
    };
    let table_name = inflection::table_name_of(&table_name.unwrap_or(struct_name.to_string()));

    let expanded = quote! {
        #struct_input

        impl #struct_name {
            pub const TABLE_NAME: &'static str = #table_name;
        }
    };
    expanded.into()
}

fn parse_attr(attr: &Attribute, key: &str) -> Option<String> {
    // 这里表示已经到了 table 宏了。
    let mut relt: Option<String> = None;
    if let Meta::Path(_) = attr.meta {
        // 只有 #[table]
        // 如果 meta 是 Path，说明括号是空的，直接跳过，保留默认表名
        return relt;
    }

    let _ = attr.parse_nested_meta(|meta| {
        if meta.path.is_ident("name") {
            // 处理 #[table(name = "users")] 或 #[table(name = users)]
            let v = meta.value()?; // 得到 = 后面的内容："users" 或 users
            if let Ok(lit_str) = v.parse::<LitStr>() {
                relt = Some(lit_str.value());
            } else if let Ok(ident) = v.parse::<syn::Ident>() {
                relt = Some(ident.to_string());
            }
        } else if let Some(ident) = meta.path.get_ident() {
            // 处理 #[table(users)] ：这里的 meta.path 就是 users
            relt = Some(ident.to_string());
        }
        Ok(())
    });
    // 处理 #[table("user")]
    // parse_nested_meta 主要是解析 key=value 或 path，
    // 对于单纯的 #[table("user")]，我们需要直接解析其内部 TokenStream
    if let Meta::List(ref list) = attr.meta {
        // 尝试看能不能直接解析出一个 LitStr
        if let Ok(lit_str) = list.parse_args::<LitStr>() {
            relt = Some(lit_str.value());
        }
    }
    relt
}
