/*
    Appellation: acme-derive <library>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
//! # acme-derive
//!
//!
extern crate proc_macro;

pub(crate) mod ast;
pub(crate) mod cmp;
pub(crate) mod utils;

use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::{parse_macro_input, Data, DataStruct, DeriveInput};

/// This macro generates a parameter struct and an enum of parameter keys.
#[proc_macro_derive(Params, attributes(param))]
pub fn params(input: TokenStream) -> TokenStream {
    // Parse the input tokens into a syntax tree
    let input = parse_macro_input!(input as DeriveInput);

    // Get the name of the struct
    let struct_name = &input.ident;
    let store_name = format_ident!("{}Key", struct_name);

    // Generate the parameter struct definition
    let _param_struct = match &input.data {
        Data::Struct(s) => match &s.fields {
            _ => {}
        },
        _ => panic!("Only structs are supported"),
    };

    // Generate the parameter keys enum
    let param_keys_enum = match &input.data {
        Data::Struct(s) => {
            let DataStruct { fields, .. } = s;

            crate::cmp::params::generate_keys(fields, &store_name)
        }
        _ => panic!("Only structs are supported"),
    };

    // Combine the generated code
    let generated_code = quote! {

        #param_keys_enum
    };

    // Return the generated code as a TokenStream
    generated_code.into()
}
