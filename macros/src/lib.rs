/*
    Appellation: acme-macros <library>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
//! # acme-macros
//!
//!
#![feature(proc_macro_span)]
extern crate proc_macro;

pub(crate) mod ast;
pub(crate) mod cmp;
pub(crate) mod diff;
pub(crate) mod grad;
pub(crate) mod ops;

pub(crate) mod gradient;

use ast::partials::*;
use proc_macro::TokenStream;
use quote::quote;
use syn::spanned::Spanned;
use syn::{parse_macro_input, Expr};

#[proc_macro_attribute]
pub fn show_streams(attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as syn::ItemFn);
    println!("attr: \"{:?}\"", &attr);
    println!("item: \"{:?}\"", &input);
    (quote! { #input }).into()
}

#[proc_macro]
pub fn show_item(item: TokenStream) -> TokenStream {
    let expr = parse_macro_input!(item as Expr);
    let span = expr.span();
    println!("Span Bytes: {:?}", span.byte_range());
    println!("Span (start, end): ({:?}, {:?})", span.start(), span.end());
    println!("Source File: {:?}", span.unwrap().source_file());
    println!("Source Text: {:?}", span.source_text());
    (quote! { #expr }).into()
}

#[proc_macro_attribute]
pub fn partial(attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as syn::ItemFn);
    println!("attr: \"{}\"", attr.to_string());
    // let result = ad::handle::item::handle_item(&input);
    // TokenStream::from(result)
    (quote! { #input }).into()
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
