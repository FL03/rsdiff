/*
    Appellation: method <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
use super::handle_expr;
use crate::ad::ops::{Methods, UnaryMethod};
use proc_macro2::TokenStream;
use quote::quote;
use std::str::FromStr;
use syn::{Expr, ExprCall, ExprMethodCall, Ident};

pub fn handle_call(expr: &ExprCall, var: &Ident) -> TokenStream {
    let ExprCall { func, args, .. } = expr;
    // 
    let df = handle_expr(&func, var);

    let mut grad = quote! { 0.0 };
    for arg in args {
        let arg = handle_expr(&arg, var);
        grad = quote! { #grad + #arg };
    }
    quote! { #df + #grad }
}

pub fn handle_method(expr: &ExprMethodCall, var: &Ident) -> TokenStream {
    let ExprMethodCall {
        args,
        method,
        receiver,
        ..
    } = expr;
    let method_name = method.clone().to_string();
    let dr = handle_expr(&receiver, var);
    if let Ok(method) = Methods::from_str(&method_name) {
        let dm = match method {
            Methods::Unary(method) => handle_unary_method(&method, &receiver, var),
        };

        return quote! { #dm * #dr };
    }
    let mut grad = quote! { 0.0 };
    for arg in args {
        let da = handle_expr(&arg, var);
        grad = quote! { #grad + #da };
    }
    quote! { #dr + #grad }
}

pub fn handle_unary_method(method: &UnaryMethod, recv: &Expr, _var: &Ident) -> TokenStream {
    match method {
        UnaryMethod::Abs => quote! { #recv / #recv.abs() },
        UnaryMethod::Cos => quote! { -#recv.sin() },
        UnaryMethod::Cosh => quote! { #recv.sinh() },
        UnaryMethod::Exp => {
            quote! {
                if #recv.is_sign_negative() {
                    -#recv.exp()
                } else {
                    #recv.exp()
                }
            }
        }
        UnaryMethod::Inverse | UnaryMethod::Recip => quote! { -#recv.powi(-2) },
        UnaryMethod::Ln => quote! { #recv.recip() },
        UnaryMethod::Sin => quote! { #recv.cos() },
        UnaryMethod::Sinh => quote! { #recv.cosh() },
        UnaryMethod::Sqrt => quote! { (2.0 * #recv.sqrt()).recip() },
        UnaryMethod::Tan => quote! { #recv.cos().powi(2).recip() },
        UnaryMethod::Tanh => quote! { #recv.cosh().powi(2).recip() },
    }
}
