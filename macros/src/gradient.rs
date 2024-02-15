/*
    Appellation: gradient <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
use proc_macro2::TokenStream;
use quote::quote;
use std::collections::HashMap;
use syn::{Expr, ExprBinary};

pub(crate) type GradientStore = HashMap<Expr, TokenStream>;

pub fn compute_gradient(expr: &Expr) -> TokenStream {
    // Initialize an empty Vec to hold the gradient values
    let mut store = Vec::new();

    // Generate code to compute the gradient of the expression with respect to each variable
    generate_gradient(expr, &mut store);

    // Convert the gradient values into a token stream
    let gradient_array = quote! { [#(#store),*] };

    // Return the generated code as a token stream
    gradient_array
}

pub fn compute_grad(expr: &Expr) -> TokenStream {
    // Initialize an empty Vec to hold the gradient values
    let mut store = HashMap::new();

    // Generate code to compute the gradient of the expression with respect to each variable
    handle_expr(expr, &mut store);

    let values = store.values().collect::<Vec<_>>();
    // Convert the gradient values into a token stream
    let gradient_array = quote! { [#(#values),*] };

    // Return the generated code as a token stream
    gradient_array
}

fn generate_gradient(expr: &Expr, store: &mut Vec<TokenStream>) {
    match expr {
        // Handle binary expressions (e.g., addition, multiplication)
        Expr::Binary(expr_binary) => {
            let left = &expr_binary.left;
            let right = &expr_binary.right;

            generate_gradient(left, store);
            generate_gradient(right, store);
        }

        // Handle constants
        Expr::Const(_) => {
            // For constants, add 0 to the gradient vector
            store.push(quote! { 0.0 });
        }
        Expr::Lit(_) => {
            // For literals, add 0 to the gradient vector
            store.push(quote! { 0.0 });
        }
        // Handle variables (identifiers)
        Expr::Path(expr_path) => {
            if expr_path.path.segments.len() != 1 {
                panic!("Unsupported path!");
            }
            let _path = &expr_path.path;
            // For variables, add 1 to the gradient vector
            store.push(quote! { 1.0 });
        }
        Expr::Reference(_) => {
            store.push(quote! { 1.0 });
        }
        _ => {
            store.push(quote! { 0.0 });
        },
    }
}

pub fn handle_expr(expr: &Expr, store: &mut HashMap<Expr, TokenStream>) -> Option<TokenStream> {
    match expr {
        Expr::Binary(expr_binary) => {
            let result = handle_binary(expr_binary, store);
            store.insert(expr.clone(), result.clone())
        
        },
        Expr::Const(_) => {
            store.insert(expr.clone(), quote! { 0.0 })
        },
        Expr::Lit(_) => {
            store.insert(expr.clone(), quote! { 0.0 })
        },
        Expr::Path(expr_path) => {
            if expr_path.path.segments.len() != 1 {
                panic!("Unsupported path!");
            }
            let _path = &expr_path.path;
            store.insert(expr.clone(), quote! { 1.0 })
        }
        Expr::Reference(_) => {
            store.insert(expr.clone(), quote! { 1.0 })
        }
        _ => panic!("Unsupported expression!"),
    }
}

pub fn handle_binary(expr: &ExprBinary, store: &mut HashMap<Expr, TokenStream>) -> TokenStream {
    use syn::BinOp;
    let left = &expr.left;
    let right = &expr.right;
    let op = &expr.op;

    let dl = handle_expr(left, store).unwrap();
    let dr = handle_expr(right, store).unwrap();

    match op {
        BinOp::Add(_) => {
            quote! {
                #dl + #dr
            }
        }
        syn::BinOp::Sub(_) => {
            quote! {
                #dl - #dr
            }
        }
        syn::BinOp::Mul(_) => {
            quote! {
                #left * #dr + #dl * #right
            }
        }
        syn::BinOp::Div(_) => {
            quote! {
                (#left * #dr - #dl * #right) / (#right * #right)
            }
        }
        _ => panic!("Unsupported binary operator!"),
    }
    
}