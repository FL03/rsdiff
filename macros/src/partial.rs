/*
    Appellation: partial <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
pub use crate::ast::PartialAst;

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
        // Differentiate parenthesized expressions
        Expr::Paren(expr_paren) => handle_partial(&expr_paren.expr, variable),
        // Differentiate variable expressions
        Expr::Path(expr_path) => {
            if expr_path.path.segments.len() != 1 {
                panic!("Unsupported path!");
            }
            let path = &expr_path.path;
            if path.segments[0].ident == *variable {
                quote! { 1.0 }
            } else {
                quote! { 0.0 }
            }
        }
        Expr::Reference(_eref) => {
            quote! { 1.0 }
        }
        // Differentiate other expressions
        _ => panic!("Unsupported expression!"),
    }
}

fn handle_binary(expr: &ExprBinary, variable: &Ident) -> TokenStream {
    use syn::BinOp;
    let left = *expr.left.clone();
    let right = *expr.right.clone();
    let op = &expr.op;

    // Differentiate left and right subexpressions
    let dl = handle_partial(&left, variable);
    let dr = handle_partial(&right, variable);

    // Apply the chain rule based on the operator
    match op {
        // Differentiate addition
        BinOp::Add(_) => {
            quote! {
                #dl + #dr
            }
        }
        // Differentiate multiplication
        BinOp::Mul(_) => {
            quote! {
                #right * #dl + #left * #dr
            }
        }
        _ => panic!("Unsupported operation!"),
    }
}
