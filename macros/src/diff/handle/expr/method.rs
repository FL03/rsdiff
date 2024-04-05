/*
    Appellation: method <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
use super::handle_expr;
use crate::ops::{BinaryOp, Methods, UnaryOp};
use proc_macro2::TokenStream;
use quote::quote;
use std::str::FromStr;
use syn::{Expr, ExprCall, ExprMethodCall, Ident};

pub fn handle_call(expr: &ExprCall, var: &Ident) -> TokenStream {
    let ExprCall { args, func, .. } = expr;
    let mut grad = quote! { 0.0 };
    for arg in args {
        let arg = handle_expr(&arg, var);
        grad = quote! { #grad + #arg };
    }

    //
    let df = handle_expr(&func, var);

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
            Methods::Binary(method) => handle_method_binary(&method, &receiver, &args[0], var),
            Methods::Unary(method) => handle_method_unary(&method, &receiver, var),
            // _ => panic!("Unsupported method"),
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

pub fn handle_method_binary(method: &BinaryOp, lhs: &Expr, rhs: &Expr, var: &Ident) -> TokenStream {
    let dl = handle_expr(&lhs, var);
    let dr = handle_expr(&rhs, var);
    println!("(dl, dr): ({}, {}) w.r.t {}", &dl, &dr, &var);
    match method {
        BinaryOp::Pow => {
            quote! {
               #rhs * #lhs.powf(#rhs - 1.0) * #dl + #lhs.ln() * #lhs.powf(#rhs) * #dr
            }
        }
    }
}
pub fn handle_method_unary(method: &UnaryOp, recv: &Expr, _var: &Ident) -> TokenStream {
    match method {
        UnaryOp::Abs => quote! { #recv / #recv.abs() },
        UnaryOp::Cos => quote! { -#recv.sin() },
        UnaryOp::Cosh => quote! { #recv.sinh() },
        UnaryOp::Exp => {
            quote! {
                if #recv.is_sign_negative() {
                    -#recv.exp()
                } else {
                    #recv.exp()
                }
            }
        }
        UnaryOp::Inverse | UnaryOp::Recip => quote! { -#recv.powi(-2) },
        UnaryOp::Ln => quote! { #recv.recip() },
        UnaryOp::Sin => quote! { #recv.cos() },
        UnaryOp::Sinh => quote! { #recv.cosh() },
        UnaryOp::Sqrt => quote! { (2.0 * #recv.sqrt()).recip() },
        UnaryOp::Tan => quote! { #recv.cos().powi(2).recip() },
        UnaryOp::Tanh => quote! { #recv.cosh().powi(2).recip() },
    }
}
