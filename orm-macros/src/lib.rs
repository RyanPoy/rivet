extern crate proc_macro;

mod char_utils;
mod col_utils;
mod common_utils;

use crate::char_utils::generate_col_column;
use crate::common_utils::parse_arg_value_from;
use proc_macro::TokenStream;
use quote::quote;
use syn::{Fields, ItemStruct, parse_macro_input};
use utils::inflection;

#[proc_macro_attribute]
pub fn table(table_args: TokenStream, item: TokenStream) -> TokenStream {
    let struct_input = &mut parse_macro_input!(item as ItemStruct);
    let struct_name = &struct_input.ident;
    let table_name = parse_arg_value_from(table_args.into(), "name")
        .unwrap_or_else(|| inflection::table_name_of(&struct_name.to_string()));

    let mut column_consts = Vec::new();

    if let Fields::Named(ref mut fields) = struct_input.fields {
        for field in &mut fields.named {
            let field_name = field.ident.as_ref().unwrap().to_string();

            if let Some(token) = generate_col_column(field, &field_name) {
                column_consts.push(token);
                field.attrs.retain(|a| !a.path().is_ident("col"));
            }
        }
    }

    let table_impl = quote! {
        impl Model for #struct_name {
            const TABLE_NAME: &'static str = #table_name;
        }
    };

    if column_consts.is_empty() {
        quote! {
            #struct_input
            #table_impl
        }
        .into()
    } else {
        let column_impl = quote! {
            #[allow(non_upper_case_globals)]
            impl #struct_name {
                #( #column_consts )*
            }
        };
        quote! {
            #struct_input
            #table_impl
            #column_impl
        }
        .into()
    }
}
