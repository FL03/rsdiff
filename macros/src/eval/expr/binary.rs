/*
    Appellation: binary <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
use super::handle_expr;
use proc_macro2::TokenStream;
use quote::quote;
use syn::{BinOp, Expr, ExprBinary, ExprParen, Ident};

pub fn handle_binary(expr: &ExprBinary, variable: &Ident) -> TokenStream {
    let ExprBinary {
        left, right, op, ..
    } = expr;

    // Differentiate left and right subexpressions
    let dl = handle_expr(&left, variable);
    let dr = handle_expr(&right, variable);

    // Apply the chain rule based on the operator
    match op {
        // Differentiate addition
        BinOp::Add(_) => {
            quote! {
                #dl + #dr
            }
        }
        BinOp::AddAssign(_) => {
            quote! {
                #dl + #dr
            }
        }
        // Differentiate division using the quotient rule
        BinOp::Div(_) => {
            quote! {
                (#dl * #right - #left * #dr) / (#right * #right)
            }
        }
        BinOp::DivAssign(_) => {
            quote! {
                (#dl * #right - #left * #dr) / (#right * #right)
            }
        }
        // Differentiate multiplication
        BinOp::Mul(star) => {
            if let Expr::Paren(inner) = *right.clone() {
                let ExprParen { expr, .. } = inner;
                if let Expr::Binary(inner) = *expr {
                    let ExprBinary {
                        left: le,
                        right: re,
                        ..
                    } = inner;
                    let pleft = ExprBinary {
                        left: left.clone(),
                        right: le.clone(),
                        op: BinOp::Mul(star.clone()),
                        attrs: vec![],
                    };
                    let pright = ExprBinary {
                        left: left.clone(),
                        right: re.clone(),
                        op: BinOp::Mul(star.clone()),
                        attrs: vec![],
                    };

                    let dl = handle_expr(&pleft.into(), variable);
                    let dr = handle_expr(&pright.into(), variable);
                    return quote! {
                        #dl + #dr
                    };
                }
            }
            if let Expr::Paren(inner) = *left.clone() {
                let ExprParen { expr, .. } = inner;
                if let Expr::Binary(inner) = *expr {
                    let ExprBinary {
                        left: le,
                        right: re,
                        ..
                    } = inner;
                    let pleft = ExprBinary {
                        left: le.clone(),
                        right: right.clone(),
                        op: BinOp::Mul(star.clone()),
                        attrs: vec![],
                    };
                    let pright = ExprBinary {
                        left: re.clone(),
                        right: right.clone(),
                        op: BinOp::Mul(star.clone()),
                        attrs: vec![],
                    };

                    let dl = handle_expr(&pleft.into(), variable);
                    let dr = handle_expr(&pright.into(), variable);
                    return quote! {
                        #dl + #dr
                    };
                }
            }
            quote! {
                #dl * #right + #dr * #left
            }
        }
        BinOp::MulAssign(_) => {
            quote! {
                #dl * #right + #dr * #left
            }
        }
        // Differentiate subtraction
        BinOp::Sub(_) => {
            quote! {
                #dl - #dr
            }
        }
        BinOp::SubAssign(_) => {
            quote! {
                #dl - #dr
            }
        }
        _ => panic!("Unsupported operation!"),
    }
}
