/*
    Appellation: unary <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
use super::handle_expr;
use proc_macro2::TokenStream;
use quote::quote;
use syn::{ExprUnary, Ident, UnOp};

pub fn handle_unary(expr: &ExprUnary, variable: &Ident) -> TokenStream {
    let dv = handle_expr(&expr.expr, variable);
    match expr.op {
        UnOp::Neg(_) => {
            quote! { -#dv }
        }
        _ => panic!("Unsupported unary operator!"),
    }
}
