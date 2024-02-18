/*
    Appellation: gradient <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
use crate::cmp::GradientStore;
use proc_macro2::TokenStream;
use quote::quote;
use syn::{Expr, ExprBinary, ExprUnary};

pub fn compute_grad(expr: &Expr) -> TokenStream {
    // Initialize an empty HashMap to hold the gradient values
    let mut store = GradientStore::new();
    // begin by computing the gradient of the expression w.r.t. itself
    // store.insert(expr.clone(), quote! { 1.0 });

    // Generate code to compute the gradient of the expression w.r.t. each variable
    handle_expr(expr, &mut store);

    store.retain_vars();

    let values = store
        .into_iter()
        .map(|(k, v)| {
            quote! { (#k, #v) }
        })
        .collect::<Vec<_>>();
    // Convert the gradient values into a token stream
    quote! { [#(#values),*] }
}

pub fn handle_expr(expr: &Expr, store: &mut GradientStore) -> TokenStream {
    match expr {
        Expr::Binary(inner) => {
            let df = binary_grad(inner, store);
            df
        }
        // Handle constants
        Expr::Const(_) => quote! { 0.0 },
        // Handle literals
        Expr::Lit(_) => quote! { 0.0 },
        Expr::Paren(inner) => handle_expr(&inner.expr, store),
        // Handle path variables (identifiers)
        Expr::Path(inner) => {
            let path = &inner.path;
            // Only considers single-segment paths; i.e., x in the expression let x = ___;
            if path.segments.len() != 1 {
                panic!("Unsupported path!");
            }
            let grad = quote! { 1.0 };
            // store.insert(node, grad.clone());
            grad
        }
        // Handle references (borrowed variables denoted with & or &mut)
        Expr::Reference(inner) => handle_expr(&inner.expr, store),
        // Handle unary expressions (e.g., negation, natural log, etc.)
        Expr::Unary(inner) => {
            // Compute the gradient of the expression
            let df = handle_unary(inner, store);

            df
        }
        // Handle other expressions
        _ => panic!("Unsupported expression!"),
    }
}

fn binary_grad(expr: &ExprBinary, store: &mut GradientStore) -> TokenStream {
    use syn::BinOp;
    // create a cloned reference to the expression
    let node: Expr = expr.clone().into();
    // let grad = store.entry(node).or_insert(quote! { 0.0 }).clone();
    let grad = store.remove(&node).unwrap_or(quote! { 0.0 });
    let ExprBinary {
        left, op, right, ..
    } = expr;

    // Recursivley compute the gradient of the left and right children
    let dl = handle_expr(left, store);
    let dr = handle_expr(right, store);
    match op {
        BinOp::Add(_) => {
            let gl = store.or_insert(*left.clone(), quote! { 0.0 });
            *gl = quote! { #gl + #dl };
            let gr = store.or_insert(*right.clone(), quote! { 0.0 });
            *gr = quote! { #gr + #dr };
        }
        BinOp::Mul(_) => {
            let gl = store.or_insert(*left.clone(), quote! { 0.0 });
            *gl = quote! { #gl + #right * #dl };
            let gr = store.or_insert(*right.clone(), quote! { 0.0 });
            *gr = quote! { #gr + #left * #dr };
        }
        _ => panic!("Unsupported binary operator!"),
    };
    grad
}

fn handle_unary(expr: &ExprUnary, store: &mut GradientStore) -> TokenStream {
    use syn::UnOp;
    handle_expr(&expr.expr, store);
    let dv = &store[&expr.expr.clone()];
    let df = match expr.op {
        UnOp::Neg(_) => {
            quote! { -#dv }
        }
        _ => panic!("Unsupported unary operator!"),
    };
    df
}
