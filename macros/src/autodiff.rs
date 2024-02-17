/*
    Appellation: autodiff <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
use proc_macro2::TokenStream;
use quote::quote;
use syn::{Expr, ExprBinary};

pub fn handle_autodiff(expr: &Expr) -> TokenStream {
    // Generate code to compute gradients based on the graph structure
    let grad = handle_expr(expr);

    grad
}

fn handle_expr(expr: &Expr) -> TokenStream {
    match expr {
        Expr::Binary(inner) => handle_binary(inner),
        Expr::Unary(_) => {
            // Implement unary expression handling
            quote! {
                // Implement unary expression handling
            }
        }
        _ => panic!("Unsupported expression!"),
    }
}

fn handle_binary(expr: &ExprBinary) -> TokenStream {
    use syn::BinOp;
    let ExprBinary {
        left, op, right, ..
    } = expr;

    let dl = handle_expr(&left);
    let dr = handle_expr(&right);
    match op {
        BinOp::Add(_) => {
            // Implement addition handling
            quote! {
                // Implement addition handling
            }
        }
        BinOp::Mul(_) => {
            // Implement multiplication handling
            quote! {
                // Implement multiplication handling
            }
        }
        _ => panic!("Unsupported binary operator!"),
    }
}
