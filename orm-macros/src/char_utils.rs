use syn::Field;

pub enum ColumnType {
    Char { length: usize },
    Text,
}

pub struct ColAttrParams {
    pub column_name: String,
    pub column_type: ColumnType,
}

pub fn parse_col_attr(field: &Field, default_name: &str) -> Option<ColAttrParams> {
    let attr = field.attrs.iter().find(|a| a.path().is_ident("col"))?;
    parse_col_from_attr(attr, default_name)
}

pub fn generate_col_column(field: &Field, default_name: &str) -> Option<proc_macro2::TokenStream> {
    use quote::quote;
    let params = parse_col_attr(field, default_name)?;

    match params.column_type {
        ColumnType::Char { length } => {
            let column_name_ident = syn::Ident::new(&params.column_name, proc_macro2::Span::call_site());
            let column_name = &params.column_name;
            Some(quote! {
                pub const #column_name_ident: crate::CharColumn = crate::CharColumn::new(#column_name, #length);
            })
        }
        ColumnType::Text => {
            None
        }
    }
}

fn parse_col_from_attr(attr: &syn::Attribute, default_name: &str) -> Option<ColAttrParams> {
    if let syn::Meta::List(meta) = &attr.meta {
        let tokens: Vec<_> = meta.tokens.clone().into_iter().collect();
        if tokens.is_empty() {
            return None;
        }

        let first = tokens.first()?;
        let column_type = match first {
            proc_macro2::TokenTree::Ident(ident) => {
                match ident.to_string().as_str() {
                    "char" => {
                        let length_str = parse_kv_arg(&tokens, "length");
                        let length = length_str
                            .and_then(|s| s.parse::<usize>().ok())
                            .unwrap_or(255);
                        ColumnType::Char { length }
                    }
                    "text" => ColumnType::Text,
                    _ => return None,
                }
            }
            _ => return None,
        };

        let name = parse_kv_arg(&tokens, "name")
            .map(|s| s.to_string())
            .unwrap_or_else(|| default_name.to_string());

        Some(ColAttrParams {
            column_name: name,
            column_type,
        })
    } else {
        None
    }
}

fn parse_kv_arg(tokens: &[proc_macro2::TokenTree], key: &str) -> Option<String> {
    let mut i = 0;
    while i < tokens.len() {
        if let proc_macro2::TokenTree::Ident(ident) = &tokens[i] {
            if ident.to_string() == key && i + 2 < tokens.len() {
                if let proc_macro2::TokenTree::Punct(ref punct) = tokens[i + 1] {
                    if punct.as_char() == '=' {
                        if let proc_macro2::TokenTree::Literal(ref lit) = tokens[i + 2] {
                            let lit_str = lit.to_string();
                            if (lit_str.starts_with('"') && lit_str.ends_with('"'))
                                || (lit_str.starts_with('\'') && lit_str.ends_with('\''))
                            {
                                return Some(lit_str[1..lit_str.len() - 1].to_string());
                            } else if let Ok(_n) = lit_str.parse::<usize>() {
                                return Some(lit_str);
                            }
                        }
                    }
                }
            }
        }
        i += 1;
    }
    None
}