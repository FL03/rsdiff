/*
    Appellation: acme-macros <library>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
//! # acme-macros
//!
//!
extern crate proc_macro;

pub(crate) mod ast;
pub(crate) mod diff;
pub(crate) mod grad;
pub(crate) mod ops;

// use ast::gradient::GradientAst;
use ast::partials::PartialAst;
use proc_macro::TokenStream;
use syn::parse_macro_input;

/// Compute the partial derivative of a given expression w.r.t a particular variable.
/// At the moment, the macro only supports expressions defined within the same scope.
///
/// # Examples
///
/// ## Compute the gradient of a simple expression
///
/// ```
/// extern crate acme_macros as macros;
///
/// use macros::autodiff;
///
/// fn main() {
///     let x = 3f64;
///     let y = 4f64;
///     let dx = autodiff!(x: x * y);
///     let dy = autodiff!(y: x * y);
///
///     assert_eq!(dx, y);
///     assert_eq!(dy, x);
/// }
/// ```
#[proc_macro]
pub fn autodiff(input: TokenStream) -> TokenStream {
    // Parse the input expression into a syntax tree
    let expr = parse_macro_input!(input as PartialAst);

    // Generate code to compute the gradient
    let result = diff::generate_autodiff(&expr);

    // Return the generated code as a token stream
    TokenStream::from(result)
}

#[proc_macro_attribute]
pub fn partial(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(item as syn::ItemFn);
    let result = grad::handle_item_fn(&ast);
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
