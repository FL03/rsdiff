/*
    Appellation: acme-macros <library>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
//! # acme-macros
//!
//!
#![feature(proc_macro_span,)]
extern crate proc_macro;


pub(crate) mod ad;
pub(crate) mod ast;
pub(crate) mod cmp;

pub(crate) mod gradient;
pub(crate) mod graph;

use ast::partials::*;
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Expr,};
use syn::spanned::Spanned;



#[proc_macro_attribute]
pub fn show_streams(attr: TokenStream, item: TokenStream) -> TokenStream {
    println!("attr: \"{}\"", attr.to_string());
    println!("item: \"{}\"", item.to_string());
    item
}

#[proc_macro]
pub fn show_item(item: TokenStream) -> TokenStream {
    
    let expr = parse_macro_input!(item as Expr);
    let span = expr.span();
    println!("Span Bytes: {:?}", span.byte_range());
    println!("Span (start, end): ({:?}, {:?})", span.start(), span.end());
    println!("Source File: {:?}", span.unwrap().source_file());
    println!("Source Text: {:?}", span.source_text());
    quote! { #expr }.into()
}

#[proc_macro]
pub fn autodiff(input: TokenStream) -> TokenStream {
    // Parse the input expression into a syntax tree
    let expr = parse_macro_input!(input as PartialAst);

    // Generate code to compute the gradient
    let result = ad::generate_autodiff(&expr);

    // Return the generated code as a token stream
    TokenStream::from(result)
}

#[proc_macro]
pub fn compute(input: TokenStream) -> TokenStream {
    use graph::Context;
    // Parse the input expression into a syntax tree
    let expr = parse_macro_input!(input as Expr);

    // Build a computational graph representing the expression
    let mut graph = Context::new();
    graph.traverse(&expr);

    // Generate code to compute gradients and return as a HashMap
    let grad = graph.backward();
    let grads = grad
        .into_iter()
        .map(|(k, v)| {
            let k = k.index();
            quote! { (#k, #v) }
        })
        .collect::<Vec<_>>();
    quote! { [#(#grads),*] }.into()
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
pub fn partial(input: TokenStream) -> TokenStream {
    // Parse the input token stream into a structured syntax tree
    let partial = parse_macro_input!(input as Partial);

    // Generate code to perform partial differentiation
    let result = ad::handle::expr::handle_expr(&partial.expr, &partial.var);

    // Return the generated code as a token stream
    TokenStream::from(result)
}

pub(crate) mod kw {
    syn::custom_keyword!(grad);

    syn::custom_keyword!(cos);
    syn::custom_keyword!(e);
    syn::custom_keyword!(ln);
    syn::custom_keyword!(sin);
    syn::custom_keyword!(tan);
}
