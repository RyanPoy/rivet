extern crate proc_macro;

mod col_utils;
mod common_utils;

use crate::common_utils::parse_arg_value_from;
use proc_macro::TokenStream;
use quote::quote;
use syn::{ItemStruct, parse_macro_input};
use utils::inflection;

#[proc_macro_attribute]
pub fn table(table_args: TokenStream, item: TokenStream) -> TokenStream {
    let struct_input = &mut parse_macro_input!(item as ItemStruct);
    let struct_name = &struct_input.ident;
    let table_name = if table_args.is_empty() {
        inflection::table_name_of(&struct_name.to_string())
    } else {
        parse_arg_value_from(table_args.into(), "name")
            .unwrap_or_else(|| inflection::table_name_of(&struct_name.to_string()))
    };

    quote! {
        #struct_input

        impl Model for #struct_name {
            const TABLE_NAME: &'static str = #table_name;
        }
    }
    .into()
}
