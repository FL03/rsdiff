/*
    Appellation: binary <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
use super::handle_expr;
use proc_macro2::{Span, TokenStream};
use quote::quote;
use syn::{BinOp, Expr, ExprBinary, ExprParen, Ident, Token};

pub fn handle_binary(expr: &ExprBinary, var: &Ident) -> TokenStream {
    let ExprBinary {
        left, right, op, ..
    } = expr;

    // Compute the partial derivative of the left expression w.r.t. the variable
    let dl = handle_expr(&left, var);
    // Compute the partial derivative of the right expression w.r.t. the variable
    let dr = handle_expr(&right, var);

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

fn foil_expr(a: &Expr, b: &ExprParen, var: &Ident) -> TokenStream {
    let ExprParen { expr, .. } = b;
    let box_a = Box::new(a.clone());
    let star = Token![*](Span::call_site());
    if let Expr::Binary(inner) = *expr.clone() {
        // Multiply the first term of the first expression by the first term of the second expression
        let pleft = ExprBinary {
            attrs: Vec::new(),
            left: box_a.clone(),
            op: BinOp::Mul(star),
            right: inner.left,
        };
        let pright = ExprBinary {
            attrs: Vec::new(),
            left: box_a,
            op: BinOp::Mul(star),
            right: inner.right,
        };
        // Create a new expression with the two new terms; (a + b) * c = a * c + b * c
        let new_expr = ExprBinary {
            attrs: Vec::new(),
            left: Box::new(pleft.into()),
            op: inner.op,
            right: Box::new(pright.into()),
        };

        // let _dl = handle_expr(&pleft.into(), var);
        // let _dr = handle_expr(&pright.into(), var);
        return handle_expr(&new_expr.into(), var);
    }
    panic!("FOILER")
}

// (a + b) * (c + d) = a * c + a * d + b * c + b * d
// (a + b) * (c - d) = a * c - a * d + b * c - b * d
fn foil(a: &ExprParen, b: &ExprParen, var: &Ident) -> TokenStream {
    let ExprParen { expr: expr_a, .. } = a;
    let ExprParen { expr: expr_b, .. } = b;
    let star = Token![*](Span::call_site());
    if let Expr::Binary(inner_a) = *expr_a.clone() {
        if let Expr::Binary(inner_b) = *expr_b.clone() {
            let al = ExprBinary {
                attrs: Vec::new(),
                left: inner_a.left.clone(),
                op: BinOp::Mul(star.clone()),
                right: inner_b.left.clone(),
            };
            let ar = ExprBinary {
                attrs: Vec::new(),
                left: inner_a.left.clone(),
                op: BinOp::Mul(star.clone()),
                right: inner_b.right.clone(),
            };
            let bl = ExprBinary {
                attrs: Vec::new(),
                left: inner_a.right.clone(),
                op: BinOp::Mul(star.clone()),
                right: inner_b.left.clone(),
            };
            let br = ExprBinary {
                attrs: Vec::new(),
                left: inner_a.right.clone(),
                op: BinOp::Mul(star.clone()),
                right: inner_b.right.clone(),
            };
            let pleft = ExprBinary {
                attrs: Vec::new(),
                left: Box::new(al.into()),
                op: inner_a.op,
                right: Box::new(ar.into()),
            };
            let pright = ExprBinary {
                attrs: Vec::new(),
                left: Box::new(bl.into()),
                op: inner_a.op,
                right: Box::new(br.into()),
            };
            let dl = handle_expr(&pleft.into(), var);
            let dr = handle_expr(&pright.into(), var);
            return quote! {
                #dl + #dr
            }
        }
    }
    panic!("FOILER")
}