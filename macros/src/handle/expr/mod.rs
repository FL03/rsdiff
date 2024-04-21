/*
    Appellation: expr <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
pub use self::{binary::*, unary::*};

pub(crate) mod binary;
pub(crate) mod unary;

use crate::ops::Methods;
use proc_macro2::TokenStream;
use quote::quote;
use syn::parse;
use syn::punctuated::Punctuated;
use syn::token::Comma;
use syn::{Expr, ExprCall, Ident};

pub fn handle_expr(expr: &Expr, variable: &Ident) -> TokenStream {
    match expr {
        // Handle differentiable arrays
        Expr::Array(inner) => {
            let grad = inner
                .elems
                .iter()
                .map(|e| parse::<Expr>(handle_expr(e, variable).into()).unwrap());
            // let _arr = ExprArray {
            //     attrs: inner.attrs.clone(),
            //     elems: Punctuated::from_iter(grad),
            //     bracket_token: inner.bracket_token,
            // };
            quote! {
                [#(#grad),*]
            }
        }
        // Handle differentiable binary operations
        Expr::Binary(inner) => handle_binary(inner, variable),
        // Handle differentiable function calls
        Expr::Call(inner) => handle_call(inner, variable),
        // Handle differentiable closures
        Expr::Closure(inner) => handle_expr(&inner.body, variable),
        // Differentiate constants
        Expr::Const(_) => {
            quote! { T::default() }
        }
        // Differentiate groups
        Expr::Group(inner) => handle_expr(&inner.expr, variable),
        // Differentiate literals
        Expr::Lit(_) => quote! { 0.0 },
        // Differentiate method calls
        Expr::MethodCall(inner) => Methods::from_method_call(inner, variable),
        // Differentiate parenthesized expressions
        Expr::Paren(inner) => handle_expr(&inner.expr, variable),
        // Differentiate variable expressions
        Expr::Path(inner) => {
            let syn::ExprPath { path, .. } = inner;
            if path.segments.len() != 1 {
                panic!("Unsupported path!");
            }
            if path.segments[0].ident == *variable {
                quote! { 1.0 }
            } else {
                quote! { 0.0 }
            }
        }
        Expr::Reference(inner) => handle_expr(&inner.expr, variable),
        // Differentiate unary expressions
        Expr::Unary(inner) => handle_unary(inner, variable),
        // Differentiate other expressions
        _ => panic!("Unsupported expression!"),
    }
}

pub fn handle_call(expr: &ExprCall, var: &Ident) -> TokenStream {
    let ExprCall { args, func, .. } = expr;

    let mut grad = quote! { 0.0 };
    for arg in args {
        let arg = handle_expr(arg, var);
        grad = quote! { #grad + #arg };
    }

    //
    let df = handle_expr(func, var);

    quote! { #df + #grad }
}

#[allow(dead_code)]
fn grad_ctx_with_args(ctx: &Box<Expr>, args: &Punctuated<Expr, Comma>, var: &Ident) -> TokenStream {
    let grad = handle_expr(ctx, var);
    let da = punctuated_grad(args, var);
    quote! { #grad + #da }
}

fn punctuated_grad(args: &Punctuated<Expr, Comma>, var: &Ident) -> TokenStream {
    args.iter()
        .map(|arg| handle_expr(arg, var))
        .fold(quote! { 0.0 }, |acc, arg| quote! { #acc + #arg })
}
