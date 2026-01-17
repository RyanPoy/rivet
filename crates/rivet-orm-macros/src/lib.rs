extern crate proc_macro;

use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use rivet_utils::inflection;
use syn::ItemStruct;

#[proc_macro_attribute]
pub fn table(table_args: TokenStream, item: TokenStream) -> TokenStream {
    // 解析 struct
    let struct_input: syn::ItemStruct = syn::parse_macro_input!(item as ItemStruct);
    let struct_name = &struct_input.ident;

    let table_name = if table_args.is_empty() {
        None
    } else {
        parse_arg_from(table_args.into(), "name")
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
