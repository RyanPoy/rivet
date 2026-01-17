extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use rivet_utils::inflection;
use syn::{DeriveInput, LitStr, parse_macro_input};

#[proc_macro_attribute]
pub fn table(attrs: TokenStream, item: TokenStream) -> TokenStream {
    // 解析 struct
    let input = parse_macro_input!(item as DeriveInput);
    let struct_name = &input.ident;

    // 解析 #[table(name = "users")]
    let mut table_name: Option<LitStr> = None;
    if !attrs.is_empty() {
        let parser = syn::meta::parser(|meta| {
            if meta.path.is_ident("name") {
                table_name = Some(meta.value()?.parse()?);
                Ok(())
            } else {
                Err(meta.error(String::from("unsupported #[table] attribute key")))
            }
        });
        parse_macro_input!(attrs with parser);
    }
    let default_table_name = LitStr::new(&struct_name.to_string(), struct_name.span());
    let table_name = table_name.unwrap_or(default_table_name);
    let table_name = inflection::table_name_of(&table_name.value());

    // 解析fields
    let fields = match &input.data {
        syn::Data::Struct(data) => &data.fields,
        _ => panic!("#[table] can only be used on structs"),
    };
    let columns: Vec<ColumnMeta> = fields.iter().map(parse_column).collect();

    let expanded = quote! {
        #input

        impl #struct_name {
            pub const TABLE_NAME: &'static str = #table_name;
        }
    };

    expanded.into()
}

struct ColumnMeta {
    rust_name: syn::Ident,
    column_name: syn::LitStr,
}

fn parse_column(field: &syn::Field) -> ColumnMeta {
    let rust_name = field.ident.clone().unwrap();
    let mut column_name: Option<syn::LitStr> = None;
    let attrs = &field.attrs;
    if !attrs.is_empty() {
        for attr in attrs {
            if !attr.path().is_ident("col") {
                continue;
            }

            let _ = attr.parse_nested_meta(|meta| {
                if meta.path.is_ident("name") {
                    column_name = Some(meta.value()?.parse()?);
                    Ok(())
                } else {
                    Err(meta.error(String::from("unsupported #[col] attribute key")))
                }
            });
        }
    }

    // 默认列名
    let default_name = {
        let snake = inflection::snake_case_of(&rust_name.to_string());
        syn::LitStr::new(&snake, rust_name.span())
    };

    let column_name = column_name.unwrap_or(default_name);

    ColumnMeta {
        rust_name,
        column_name,
    }
}
