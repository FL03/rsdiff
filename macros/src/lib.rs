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

use ast::partials::PartialAst;
use proc_macro::TokenStream;
use syn::parse_macro_input;

/// Compute the partial derivative of a given expression w.r.t a particular variable.
/// At the moment, the macro only supports expressions defined within the same scope.
///
/// # Examples
///
/// ### Basic arithmetic
///
/// ```
/// extern crate acme_macros as macros;
///
/// use macros::autodiff;
///
/// fn main() {
///     let x = 3f64;
///     let y = 4f64;
///
///     assert_eq!(y, autodiff!(x: x * y));
///     assert_eq!(x, autodiff!(y: x * y));
///     assert_eq!(1f64, autodiff!(x: x + y));
/// }
/// ```
///
/// ### Trigonometric functions
///
/// ```
/// extern crate acme_macros as macros;
///
/// use macros::autodiff;
///
/// fn main() {
///     let x = 2f64;
///     assert_eq!(autodiff!(x: x.cos()), -x.sin());
///     assert_eq!(autodiff!(x: x.sin()), x.cos());
///     assert_eq!(autodiff!(x: x.tan()), x.cos().powi(2).recip());
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
    syn::custom_keyword!(exp);
    syn::custom_keyword!(ln);
    syn::custom_keyword!(sin);
    syn::custom_keyword!(tan);
}
