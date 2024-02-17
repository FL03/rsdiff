/*
    Appellation: expr <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
pub use self::{binary::*, method::*, unary::*};

pub(crate) mod binary;
pub(crate) mod method;
pub(crate) mod unary;

use proc_macro2::TokenStream;
use quote::quote;
use syn::{Expr, Ident};

pub fn handle_expr(expr: &Expr, variable: &Ident) -> TokenStream {
    match expr {
        Expr::Binary(inner) => handle_binary(inner, variable),
        // Handle differentiable function calls
        Expr::Call(inner) => handle_call(inner, variable),
        // Handle differentiable closures
        Expr::Closure(inner) => handle_expr(&inner.body, variable),
        // Differentiate constants
        Expr::Const(_) => quote! { 0.0 },
        // Differentiate groups
        Expr::Group(inner) => handle_expr(&inner.expr, variable),
        // Differentiate literals
        Expr::Lit(_) => quote! { 0.0 },
        // Differentiate method calls
        Expr::MethodCall(inner) => handle_method(inner, variable),
        // Differentiate parenthesized expressions
        Expr::Paren(inner) => handle_expr(&inner.expr, variable),
        // Differentiate variable expressions
        Expr::Path(inner) => {
            if inner.path.segments.len() != 1 {
                panic!("Unsupported path!");
            }
            let path = &inner.path;
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

