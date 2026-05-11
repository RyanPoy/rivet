use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use syn::{Ident, ItemStruct, Type};

pub struct ColumnMeta {
    pub ident: Ident,
    pub name: String,
    pub tp: Type,
}

pub fn expand_columns_metadata(struct_input: &ItemStruct, metas: Vec<ColumnMeta>, table_name: &str) -> TokenStream2 {
    let column_consts = metas.iter().map(|m| {
        let field_ident = &m.ident;
        let column_name = &m.name;
        let column_type = get_column_type(&m.tp);

        quote! {
            pub const #field_ident: crate::Col<#column_type> = crate::Col::new(#column_name, crate::Table{name: #table_name});
        }
    });

    let struct_name = &struct_input.ident;
    quote! {
        #[allow(non_upper_case_globals)]
        impl #struct_name {
            pub const TABLE_NAME: &'static str = #table_name;
            #( #column_consts )*
        }
    }
}

fn get_column_type(ty: &Type) -> Type {
    match ty {
        Type::Reference(type_ref) => {
            if let syn::Type::Path(ref tp) = *type_ref.elem {
                if tp.path.is_ident("str") {
                    return syn::parse_quote!(String);
                }
            }
            get_column_type(&type_ref.elem)
        },
        Type::Path(tp) => {
            let last_segment = tp.path.segments.last().unwrap();
            if last_segment.ident == "Option" {
                if let syn::PathArguments::AngleBracketed(args) = &last_segment.arguments {
                    if let Some(syn::GenericArgument::Type(inner_ty)) = args.args.first() {
                        return get_column_type(inner_ty);
                    }
                }
            }
            ty.clone()
        },
        _ => ty.clone(),
    }
}
