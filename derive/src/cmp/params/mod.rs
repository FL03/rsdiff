/*
    Appellation: params <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
use crate::utils::capitalize_first;
use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use syn::{Fields, FieldsNamed, Ident, Variant};

pub fn generate_keys(fields: &Fields, name: &Ident) -> TokenStream {
    match fields {
        Fields::Named(inner) => handle_named_fields(inner, name),
        _ => panic!("Only named fields are supported"),
    }
}

fn handle_named_fields(fields: &FieldsNamed, name: &Ident) -> TokenStream {
    let FieldsNamed { named, .. } = fields;
    let fields_str = named.iter().cloned().map(|field| field.ident.unwrap());
    let variants = named.iter().cloned().map(|field| {
        let ident = field.ident.unwrap();
        let variant_ident = format_ident!("{}", capitalize_first(&ident.to_string()));
        Variant {
            attrs: vec![],
            ident: variant_ident,
            fields: Fields::Unit,
            discriminant: None,
        }
    });

    quote! {
        #[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]

        pub enum #name {
            #(#variants),*
        }
    }
}
