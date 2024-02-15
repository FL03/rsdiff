/*
    Appellation: acme-macros <library>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
//! # acme-macros
//!
//!
extern crate proc_macro;
use proc_macro::TokenStream;
use proc_macro2::TokenStream as Ts;
use quote::quote;
use syn::{parse_macro_input, Expr};
use syn::{Data, DeriveInput, Fields};

pub(crate) mod cmp;

pub(crate) mod gradient;
pub(crate) mod partial;

use partial::PartialDerivative;

#[proc_macro]
pub fn express(item: TokenStream) -> TokenStream {
    let input = Ts::from(item);
    // let output = parse!(input as Expr);
    println!("item: \"{:?}\"", &input.to_string());
    TokenStream::from(quote! { #input })
}

#[proc_macro_attribute]
pub fn show_streams(attr: TokenStream, item: TokenStream) -> TokenStream {
    println!("attr: \"{}\"", attr.to_string());
    println!("item: \"{}\"", item.to_string());
    item
}

#[proc_macro]
pub fn grad(input: TokenStream) -> TokenStream {
    // Parse the input expression into a syntax tree
    let expr = parse_macro_input!(input as Expr);

    // Generate code to compute the gradient
    let result = gradient::compute_grad(&expr);

    // Return the generated code as a token stream
    TokenStream::from(result)
}

#[proc_macro]
pub fn gradient(input: TokenStream) -> TokenStream {
    // Parse the input expression into a syntax tree
    let expr = parse_macro_input!(input as Expr);

    // Generate code to compute the gradient
    let result = gradient::compute_gradient(&expr);

    // Return the generated code as a token stream
    TokenStream::from(result)
}

#[proc_macro]
pub fn partial(input: TokenStream) -> TokenStream {
    // Parse the input token stream into a structured syntax tree
    let PartialDerivative { expr, variable } = parse_macro_input!(input as PartialDerivative);

    // Generate code to perform partial differentiation
    let result = partial::handle_partial(&expr, &variable);

    // Return the generated code as a token stream
    TokenStream::from(result)
}

#[proc_macro]
pub fn param(input: TokenStream) -> TokenStream {
    // Parse the input tokens into a syntax tree
    let input = parse_macro_input!(input as DeriveInput);

    // Get the name of the struct
    let struct_name = &input.ident;

    // Generate the parameter struct definition
    let param_struct = match &input.data {
        Data::Struct(s) => match &s.fields {
            Fields::Named(fields) => {
                let field_names = fields.named.iter().map(|f| f.ident.clone());
                let fn2 = field_names.clone();

                quote! {
                    impl #struct_name {
                        pub fn new(#(#field_names: Parameter),*) -> Self {
                            #struct_name {
                                #(
                                    #fn2,
                                )*
                            }
                        }
                    }
                }
            }
            _ => panic!("Only named fields are supported"),
        },
        _ => panic!("Only structs are supported"),
    };

    // Generate the parameter keys enum
    let param_keys_enum = match &input.data {
        Data::Struct(s) => match &s.fields {
            Fields::Named(fields) => {
                let field_names = fields.named.iter().map(|f| &f.ident);
                let field_names_str = field_names.clone().map(|ident| {
                    let ident_str = ident.as_ref().unwrap().to_string();
                    quote! { #ident_str }
                });

                quote! {
                    #[derive(Debug, PartialEq, Eq, Hash)]
                    pub enum #struct_name.keys {
                        #(
                            #field_names,
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
        #param_struct
        #param_keys_enum
    };

    // Return the generated code as a TokenStream
    generated_code.into()
}
