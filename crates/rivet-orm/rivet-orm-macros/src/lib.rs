extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput, LitStr};

#[proc_macro_attribute]
pub fn table(attr: TokenStream, item: TokenStream) -> TokenStream {
    // 解析 #[table(name = "users")]
    let mut table_name: Option<LitStr> = None;

    let parser = syn::meta::parser(|meta| {
        if meta.path.is_ident("name") {
            table_name = Some(meta.value()?.parse()?);
            Ok(())
        } else {
            Err(meta.error(String::from("unsupported attribute key")))
        }
    });

    parse_macro_input!(attr with parser);

    let table_name =
        table_name.expect("expected #[table(name = \"...\")]");

    // 解析 struct
    let input = parse_macro_input!(item as DeriveInput);
    let struct_name = &input.ident;

    let expanded = quote! {
        #input

        impl #struct_name {
            pub fn table_name() -> &'static str {
                #table_name
            }
        }
    };

    expanded.into()
}