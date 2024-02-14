/*
    Appellation: acme-macros <library>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
//! # acme-macros
//!
//!
extern crate proc_macro;
use proc_macro::TokenStream;
use proc_macro2::TokenStream as Ts;
use quote::quote;
use syn::{parse_macro_input, Expr, Ident};
use syn::{Data, DeriveInput, Fields};

pub(crate) mod cmp;

use cmp::PartialDerivative;

#[proc_macro]
pub fn express(item: TokenStream) -> TokenStream {
    let input = Ts::from(item);
    // let output = parse!(input as Expr);
    println!("item: \"{:?}\"", &input.to_string());
    TokenStream::from(quote! { #input })
}

#[proc_macro_attribute]
pub fn show_streams(attr: TokenStream, item: TokenStream) -> TokenStream {
    println!("attr: \"{}\"", attr.to_string());
    println!("item: \"{}\"", item.to_string());
    item
}

#[proc_macro]
pub fn differentiate(input: TokenStream) -> TokenStream {
    // Parse the input expression into a syntax tree
    let expr = parse_macro_input!(input as Expr);

    // Generate code to perform automatic differentiation
    let result = match_differentiate(&expr);

    // Return the generated code as a token stream
    TokenStream::from(result)
}

fn match_differentiate(expr: &Expr) -> Ts {
    match expr {
        Expr::Assign(expr_assign) => {
            let left = &expr_assign.left;
            let right = &expr_assign.right;

            // Differentiate the right subexpression
            let right_diff = match_differentiate(right);

            // Return the differentiated expression
            quote! {
                {
                    let right_diff = #right_diff;
                    #left = right_diff;
                }
            }
        }
        Expr::Binary(expr_binary) => {
            let left = &expr_binary.left;
            let right = &expr_binary.right;
            let op = &expr_binary.op;

            // Differentiate left and right subexpressions
            let left_diff = match_differentiate(left);
            let right_diff = match_differentiate(right);

            // Apply the chain rule based on the operator
            match op {
                // Differentiate addition and subtraction
                syn::BinOp::Add(_plus) => {
                    quote! {
                        {
                            let left_diff = #left_diff;
                            let right_diff = #right_diff;
                            left_diff + right_diff
                        }
                    }
                }
                // Differentiate multiplication and division
                syn::BinOp::Mul(_) => {
                    quote! {
                        {
                            let left_diff = #left_diff;
                            let right_diff = #right_diff;
                            left_diff * #right + #left * right_diff
                        }
                    }
                }
                _ => panic!("Unsupported operator!"),
            }
        }
        // Differentiate literal expressions (constants)
        Expr::Const(_) => quote! { 0.0 },
        // Differentiate literal expressions (constants)
        Expr::Lit(_) => quote! { 0.0 },
        Expr::Reference(_) => quote! { 1.0 },
        Expr::Path(_) => quote! { 1.0 },
        _ => panic!("Unsupported expression!"),
    }
}

#[proc_macro]
pub fn partial(input: TokenStream) -> TokenStream {
    // Parse the input token stream into a syntax tree representing the expression and variable
    let PartialDerivative { expr, variable } = parse_macro_input!(input as PartialDerivative);

    // Generate code to perform partial differentiation
    let result = match_partial_differentiate(&expr, &variable);

    // Return the generated code as a token stream
    TokenStream::from(result)
}

fn match_partial_differentiate(expr: &Expr, variable: &Ident) -> proc_macro2::TokenStream {
    match expr {
        Expr::Binary(expr_binary) => {
            let left = &expr_binary.left;
            let right = &expr_binary.right;
            let op = &expr_binary.op;

            // Differentiate left and right subexpressions
            let left_diff = match_partial_differentiate(left, variable);
            let right_diff = match_partial_differentiate(right, variable);

            // Apply the chain rule based on the operator
            match op {
                // Differentiate addition
                syn::BinOp::Add(_) => {
                    quote! {
                        {
                            if #left == #variable {
                                #left_diff
                            } else {
                                #right_diff
                            }
                        }
                    }
                }
                // Differentiate multiplication
                syn::BinOp::Mul(_) => {
                    quote! {
                        {
                            let left_diff = #left_diff;
                            let right_diff = #right_diff;
                            #left * right_diff + left_diff * #right
                        }
                    }
                }
                _ => panic!("Unsupported operation!"),
            }
        }
        // Differentiate variable expressions
        Expr::Path(expr_path)
        
            if expr_path.path.segments.len() == 1
                && expr_path.path.segments[0].ident == *variable =>
        {
            quote! { 1.0 } // The derivative of the variable with respect to itself is 1
        }
        // Differentiate other expressions
        _ => quote! { 0.0 }, // The derivative of anything else is 0
    }
}

#[proc_macro]
pub fn gradient(input: TokenStream) -> TokenStream {
    // Parse the input expression into a syntax tree
    let expr = parse_macro_input!(input as Expr);

    // Generate code to compute the gradient
    let result = compute_gradient(&expr);

    // Return the generated code as a token stream
    TokenStream::from(result)
}

fn compute_gradient(expr: &Expr) -> Ts {
    // Initialize an empty Vec to hold the gradient values
    let mut gradient_values = Vec::new();

    // Generate code to compute the gradient of the expression with respect to each variable
    generate_gradient(expr, &mut gradient_values);

    // Convert the gradient values into a token stream
    let gradient_array = quote! { [#(#gradient_values),*] };

    // Return the generated code as a token stream
    gradient_array
}

fn generate_gradient(expr: &Expr, gradient_values: &mut Vec<Ts>) {
    match expr {
        // Handle binary expressions (e.g., addition, multiplication)
        Expr::Binary(expr_binary) => {
            let left = &expr_binary.left;
            let right = &expr_binary.right;

            // Recursively compute gradient for left and right subexpressions
            generate_gradient(left, gradient_values);
            generate_gradient(right, gradient_values);
        }

        // Handle literals (constants)
        Expr::Const(_) => {
            // For constants, add 0 to the gradient vector
            gradient_values.push(quote! { 0.0 });
        }
        Expr::Lit(_) => {
            // For literals, add 0 to the gradient vector
            gradient_values.push(quote! { 0.0 });
        }
        // Handle variables (identifiers)
        Expr::Path(expr_path) => {
            if expr_path.path.segments.len() != 1 {
                panic!("Unsupported path!");
            }
            let _path = &expr_path.path;
            // For variables, add 1 to the gradient vector
            gradient_values.push(quote! { 1.0 });
        }
        Expr::Reference(_) => {
            gradient_values.push(quote! { 1.0 });
        }
        _ => panic!("Unsupported expression!"),
    }
}
#[proc_macro]
pub fn param(input: TokenStream) -> TokenStream {
    // Parse the input tokens into a syntax tree
    let input = parse_macro_input!(input as DeriveInput);

    // Get the name of the struct
    let struct_name = &input.ident;

    // Generate the parameter struct definition
    let param_struct = match &input.data {
        Data::Struct(s) => match &s.fields {
            Fields::Named(fields) => {
                let field_names = fields.named.iter().map(|f| f.ident.clone());
                let fn2 = field_names.clone();

                quote! {
                    impl #struct_name {
                        pub fn new(#(#field_names: Parameter),*) -> Self {
                            #struct_name {
                                #(
                                    #fn2,
                                )*
                            }
                        }
                    }
                }
            }
            _ => panic!("Only named fields are supported"),
        },
        _ => panic!("Only structs are supported"),
    };

    // Generate the parameter keys enum
    let param_keys_enum = match &input.data {
        Data::Struct(s) => match &s.fields {
            Fields::Named(fields) => {
                let field_names = fields.named.iter().map(|f| &f.ident);
                let field_names_str = field_names.clone().map(|ident| {
                    let ident_str = ident.as_ref().unwrap().to_string();
                    quote! { #ident_str }
                });

                quote! {
                    #[derive(Debug, PartialEq, Eq, Hash)]
                    pub enum #struct_name.keys {
                        #(
                            #field_names,
                        )*
                    }
                }
            }
            _ => panic!("Only named fields are supported"),
        },
        _ => panic!("Only structs are supported"),
    };

    // Combine the generated code
    let generated_code = quote! {
        #param_struct
        #param_keys_enum
    };

    // Return the generated code as a TokenStream
    generated_code.into()
}