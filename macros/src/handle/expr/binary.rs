/*
    Appellation: binary <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
use super::handle_expr;
use crate::{foil, foil_expr};
use proc_macro2::TokenStream;
use quote::quote;
use syn::{BinOp, Expr, ExprBinary, Ident};

pub fn handle_binary(expr: &ExprBinary, var: &Ident) -> TokenStream {
    let ExprBinary {
        left, right, op, ..
    } = expr;

    // Compute the partial derivative of the left expression w.r.t. the variable
    let dl = handle_expr(left, var);
    // Compute the partial derivative of the right expression w.r.t. the variable
    let dr = handle_expr(right, var);

    // Apply the chain rule based on the operator
    match op {
        // Differentiate addition
        BinOp::Add(_) | BinOp::AddAssign(_) => {
            quote! {
                #dl + #dr
            }
        }
        // Differentiate division using the quotient rule
        BinOp::Div(_) | BinOp::DivAssign(_) => {
            quote! {
                (#right * #dl - #left * #dr) / (#right * #right)
            }
        }
        // Differentiate multiplication
        BinOp::Mul(_) | BinOp::MulAssign(_) => {
            if let Expr::Paren(pl) = *left.clone() {
                if let Expr::Paren(pr) = *right.clone() {
                    foil(&pl, &pr, var)
                } else {
                    foil_expr(right, &pl, var)
                }
            } else if let Expr::Paren(pr) = *right.clone() {
                if let Expr::Paren(pl) = *left.clone() {
                    foil(&pl, &pr, var)
                } else {
                    foil_expr(left, &pr, var)
                }
            } else {
                quote! {
                    #dl * #right + #dr * #left
                }
            }
        }
        // Differentiate subtraction
        BinOp::Sub(_) | BinOp::SubAssign(_) => {
            quote! {
                #dl - #dr
            }
        }
        _ => panic!("Unsupported operation!"),
    }
}
