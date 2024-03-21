/*
    Appellation: acme-macros <library>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
//! # acme-macros
//!
//!
extern crate proc_macro as pm;

pub(crate) mod ast;
pub(crate) mod cmp;
pub(crate) mod diff;
pub(crate) mod grad;
pub(crate) mod ops;

pub(crate) mod gradient;

use ast::gradient::GradientAst;
use ast::partials::PartialAst;
use pm::TokenStream;
use syn::{parse_macro_input, Expr};

#[proc_macro_attribute]
pub fn partial(_attr: TokenStream, item: TokenStream) -> TokenStream {
    // let attr = parse_macro_input!(attr as syn::Attribute);
    // let item = parse_macro_input!(item as syn::ItemFn);
    // let ast = ast::gradient::GradientAst::new(attr, item);
    let ast = parse_macro_input!(item as GradientAst);
    let result = grad::gradient(&ast);
    TokenStream::from(result)
}

#[proc_macro]
pub fn autodiff(input: TokenStream) -> TokenStream {
    // Parse the input expression into a syntax tree
    let expr = parse_macro_input!(input as PartialAst);

    // Generate code to compute the gradient
    let result = diff::generate_autodiff(&expr);

    // Return the generated code as a token stream
    TokenStream::from(result)
}

#[proc_macro]
pub fn gradient(input: TokenStream) -> TokenStream {
    // Parse the input expression into a syntax tree
    let expr = parse_macro_input!(input as Expr);

    // Generate code to compute the gradient
    let result = gradient::compute_grad(&expr);

    // Return the generated code as a token stream
    TokenStream::from(result)
}

pub(crate) mod kw {
    syn::custom_keyword!(eval);
    syn::custom_keyword!(grad);

    syn::custom_keyword!(cos);
    syn::custom_keyword!(e);
    syn::custom_keyword!(ln);
    syn::custom_keyword!(sin);
    syn::custom_keyword!(tan);
}
