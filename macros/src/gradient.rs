/*
    Appellation: gradient <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
use proc_macro2::TokenStream;
use quote::quote;
use std::collections::HashMap;
use syn::{Expr, ExprBinary, ExprUnary};

pub(crate) type GradientStack = Vec<(Expr, TokenStream)>;
pub(crate) type GradientStore = HashMap<Expr, TokenStream>;

pub fn compute_grad(expr: &Expr) -> TokenStream {
    // Initialize an empty HashMap to hold the gradient values
    let mut store = HashMap::new();
    // begin by computing the gradient of the expression w.r.t. itself
    // store.insert(expr.clone(), quote! { 1.0 });

    // Generate code to compute the gradient of the expression w.r.t. each variable
    handle_expr(expr, &mut store);

    store.retain(|k, _v| matches!(k, Expr::Path(_) | Expr::Reference(_)));

    let values = store
        .into_iter()
        .map(|(k, v)| {
            quote! { (#k, #v) }
        })
        .collect::<Vec<_>>();
    // Convert the gradient values into a token stream
    quote! { [#(#values),*] }
}

pub fn handle_expr(expr: &Expr, store: &mut HashMap<Expr, TokenStream>) -> Option<TokenStream> {
    let node = expr.clone();

    match expr {
        Expr::Binary(binary) => {
            let df = binary_grad(binary, store);
            Some(df)
        }
        // Handle constants
        Expr::Const(_) => Some(quote! { 0.0 }),
        // Handle literals
        Expr::Lit(_) => Some(quote! { 0.0 }),
        Expr::Paren(paren) => handle_expr(&paren.expr, store),
        // Handle path variables (identifiers)
        Expr::Path(expr_path) => {
            // Only considers single-segment paths; i.e., x in the expression let x = ___;
            if expr_path.path.segments.len() != 1 {
                panic!("Unsupported path!");
            }
            let _path = &expr_path.path;
            let grad = quote! { 1.0 };
            store.insert(node, grad.clone());
            Some(grad)
        }
        // Handle references (borrowed variables denoted with & or &mut)
        Expr::Reference(_) => {
            let grad = quote! { 1.0 };
            store.insert(node, grad.clone());
            Some(grad)
        }
        // Handle unary expressions (e.g., negation, natural log, etc.)
        Expr::Unary(unary) => {
            // Compute the gradient of the expression
            let df = handle_unary(unary, store);

            Some(df)
        }
        // Handle other expressions
        _ => panic!("Unsupported expression!"),
    }
}

fn binary_grad(expr: &ExprBinary, store: &mut HashMap<Expr, TokenStream>) -> TokenStream {
    use syn::BinOp;
    // create a cloned reference to the expression
    let node: Expr = expr.clone().into();
    // let grad = store.entry(node).or_insert(quote! { 0.0 }).clone();
    let grad = store.remove(&node).unwrap_or(quote! { 0.0 });
    let ExprBinary { left, op, right, .. } = expr;

    // Recursivley compute the gradient of the left and right children
    let dl = handle_expr(left, store).unwrap_or(quote! { 0.0 }); 
    let dr = handle_expr(right, store).unwrap_or(quote! { 0.0 });
    match op {
        BinOp::Add(_) => {
            let gl = store.entry(*left.clone()).or_insert(quote! { 0.0 });
            *gl = quote! { #grad + #dl };
            let gr = store.entry(*right.clone()).or_insert(quote! { 0.0 });
            *gr = quote! { #grad + #dr };
        }
        BinOp::Mul(_) => {
            let gl = store.entry(*left.clone()).or_insert(quote! { 0.0 });
            *gl = quote! { #right * #dl + #grad };
            let gr = store.entry(*right.clone()).or_insert(quote! { 0.0 });
            *gr = quote! { #grad + #left * #dr };
        }
        _ => panic!("Unsupported binary operator!"),
    };
    grad
}

fn handle_unary(expr: &ExprUnary, store: &mut HashMap<Expr, TokenStream>) -> TokenStream {
    use syn::UnOp;
    handle_expr(&expr.expr, store);
    let dv = store.get(&expr.expr).unwrap_or(&quote! { 0.0 }).clone();
    let df = match expr.op {
        UnOp::Neg(_) => {
            quote! { -#dv }
        }
        _ => panic!("Unsupported unary operator!"),
    };
    df
}
