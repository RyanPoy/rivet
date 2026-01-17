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

    let mut table_name = struct_name.to_string(); // 一开始是默认名
    for attr in &struct_input.attrs {
        // #[table]
        // #[table("users")]
        // #[table(users)]
        // #[table(name="users")]
        // #[table(name=users)]

        if attr.path().is_ident("table") {

            // 这里表示已经到了 table 宏了。
            if let Meta::Path(_) = attr.meta {
                // 只有 #[table]
                // 如果 meta 是 Path，说明括号是空的，直接跳过，保留默认表名
                continue;
            }
            let _ = attr.parse_nested_meta(|meta| {
                if meta.path.is_ident("name") {
                    // 处理 #[table(name = "users")] 或 #[table(name = users)]
                    let v = meta.value()?; // 得到 = 后面的内容："users" 或 users
                    if let Ok(lit_str) = v.parse::<LitStr>() {
                        table_name = lit_str.value();
                    } else if let Ok(ident) = v.parse::<syn::Ident>() {
                        table_name = ident.to_string();
                    }
                } else if let Some(ident) = meta.path.get_ident() {
                    // 处理 #[table(users)] ：这里的 meta.path 就是 users
                    table_name = ident.to_string();
                }
                Ok(())
            });
            // 处理 #[table("user")]
            // parse_nested_meta 主要是解析 key=value 或 path，
            // 对于单纯的 #[table("user")]，我们需要直接解析其内部 TokenStream
            if let Meta::List(ref list) = attr.meta {
                // 尝试看能不能直接解析出一个 LitStr
                if let Ok(lit_str) = list.parse_args::<LitStr>() {
                    table_name = lit_str.value();
                }
            }
        }
    }
    let table_name = inflection::table_name_of(&table_name);

    let expanded = quote! {
        #struct_input

        impl #struct_name {
            pub const TABLE_NAME: &'static str = #table_name;
        }
    };
    expanded.into()
}
//
// fn parse_attr<'a>(attr: &Attribute, key: &'a str, default_value: &'a str) -> &'a str {
//     "abc"
// }
