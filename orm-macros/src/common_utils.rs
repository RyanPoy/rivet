use proc_macro2::TokenStream as TokenStream2;
use syn::{Ident, LitStr, meta, parse::Parser, parse2};

pub fn parse_arg_value_from(args: TokenStream2, arg_name: &str) -> Option<String> {
    if args.is_empty() {
        return None;
    }
    if let Ok(lit) = parse2::<LitStr>(args.clone()) {
        return Some(lit.value());
    }
    if let Ok(ident) = parse2::<Ident>(args.clone()) {
        return Some(ident.to_string());
    }

    let mut result = None;
    let parser = meta::parser(|meta| {
        if meta.path.is_ident(arg_name) {
            let value = meta.value()?;
            if let Ok(lit) = value.parse::<LitStr>() {
                result = Some(lit.value());
            } else if let Ok(ident) = value.parse::<Ident>() {
                result = Some(ident.to_string());
            }
        }
        Ok(())
    });
    let _ = parser.parse2(args);
    result
}
