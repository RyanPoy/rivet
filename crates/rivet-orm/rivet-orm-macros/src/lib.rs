extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use rivet_utils::inflection;
use syn::{DeriveInput, LitStr, parse_macro_input};

#[proc_macro_attribute]
pub fn table(attr: TokenStream, item: TokenStream) -> TokenStream {
    // 解析 struct
    let input = parse_macro_input!(item as DeriveInput);
    let struct_name = &input.ident;

    // 解析 #[table(name = "users")]
    let mut table_name: Option<LitStr> = None;
    if !attr.is_empty() {
        let parser = syn::meta::parser(|meta| {
            if meta.path.is_ident("name") {
                table_name = Some(meta.value()?.parse()?);
                Ok(())
            } else {
                Err(meta.error(String::from("unsupported attribute key")))
            }
        });
        parse_macro_input!(attr with parser);
    }
    let default_table_name = LitStr::new(&struct_name.to_string(), struct_name.span());
    let table_name = table_name.unwrap_or(default_table_name);
    let table_name = inflection::table_name_of(&table_name.value());

    let expanded = quote! {
        #input

        impl #struct_name {
            pub const TABLE_NAME: &'static str = #table_name;
        }
    };

    expanded.into()
}
