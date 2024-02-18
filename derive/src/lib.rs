/*
    Appellation: acme-derive <library>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
//! # acme-derive
//!
//!
extern crate proc_macro;
use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::{parse_macro_input, Data, DeriveInput, Fields, Variant};

fn capitalize_first(s: &str) -> String {
    s.chars()
        .take(1)
        .flat_map(|f| f.to_uppercase())
        .chain(s.chars().skip(1))
        .collect()
}

#[proc_macro_derive(AnswerFn)]
pub fn derive_answer_fn(_item: TokenStream) -> TokenStream {
    "fn answer() -> u32 { 42 }".parse().unwrap()
}

#[proc_macro_derive(HelperAttr, attributes(helper))]
pub fn derive_helper_attr(_item: TokenStream) -> TokenStream {
    TokenStream::new()
}

#[proc_macro_derive(Params, attributes(param))]
pub fn params(input: TokenStream) -> TokenStream {
    // Parse the input tokens into a syntax tree
    let input = parse_macro_input!(input as DeriveInput);

    // Get the name of the struct
    let struct_name = &input.ident;
    let store_name = format_ident!("{}Key", struct_name);

    // Generate the parameter struct definition
    let param_struct = match &input.data {
        Data::Struct(s) => match &s.fields {
            _ => {}
        },
        _ => panic!("Only structs are supported"),
    };

    // Generate the parameter keys enum
    let param_keys_enum = match &input.data {
        Data::Struct(s) => match &s.fields {
            Fields::Named(fields) => {
                let field_names = fields.named.iter().map(|f| &f.ident);
                let varaints = field_names.clone().map(|ident| {
                    let ident_str = ident.as_ref().unwrap().to_string();
                    let ident_str = format_ident!("{}", capitalize_first(&ident_str));
                    Variant {
                        attrs: vec![],
                        ident: ident_str,
                        fields: Fields::Unit,
                        discriminant: None,
                    }
                });
                let varaints_str = varaints.clone().map(|v| v.ident);

                quote! {
                    #[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd,)]
                    pub enum #store_name {
                        #(
                            #varaints,
                        )*
                    }
                }
            }
            _ => panic!("Only named fields are supported"),
        },
        _ => panic!("Only structs are supported"),
    };

    // Combine the generated code
    let generated_code = quote! {
        // #param_struct
        #param_keys_enum
    };

    // Return the generated code as a TokenStream
    generated_code.into()
}
