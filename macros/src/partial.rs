/*
    Appellation: partial <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
pub use crate::ast::PartialDerivative;

use proc_macro2::TokenStream;
use quote::quote;
use syn::{Expr, ExprBinary, Ident};

pub fn handle_partial(expr: &Expr, variable: &Ident) -> TokenStream {
    match expr {
        Expr::Binary(expr_binary) => handle_binary(expr_binary, variable),
        // Differentiate constants
        Expr::Const(_) => quote! { 0.0 },
        Expr::Group(expr_group) => handle_partial(&expr_group.expr, variable),
        // Differentiate literals
        Expr::Lit(_) => quote! { 0.0 },
        // Differentiate variable expressions
        Expr::Path(expr_path)
            if expr_path.path.segments.len() == 1
                && expr_path.path.segments[0].ident == *variable =>
        {
            quote! { 1.0 } // The derivative of the variable with respect to itself is 1
        }
        Expr::Reference(_eref) => {
            quote! { 1.0 }
        }
        Expr::Tuple(expr_tuple) => {
            let mut result = quote! {};
            for expr in &expr_tuple.elems {
                let diff = handle_partial(expr, variable);
                result = quote! { #result + #diff };
            }
            result
        }
        // Differentiate other expressions
        _ => quote! { 0.0 }, // The derivative of anything else is 0
    }
}

fn handle_binary(expr: &ExprBinary, variable: &Ident) -> TokenStream {
    use syn::BinOp;
    let left = &expr.left;
    let right = &expr.right;
    let op = &expr.op;

    // Differentiate left and right subexpressions
    let dl = handle_partial(left, variable);
    let dr = handle_partial(right, variable);

    // Apply the chain rule based on the operator
    match op {
        // Differentiate addition
        BinOp::Add(_) => {
            quote! {
                {
                    if #left == #variable {
                        #dl
                    } else {
                        #dr
                    }
                }
            }
        }
        // Differentiate multiplication
        BinOp::Mul(_) => {
            quote! {
                {
                    if #left == #variable {
                        #dl * #right
                    } else {
                        #left * #dr
                    }
                }
            }
        }
        _ => panic!("Unsupported operation!"),
    }
}
