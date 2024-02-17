/*
    Appellation: method <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
use super::handle_expr;
use crate::kw;
use proc_macro2::TokenStream;
use quote::quote;
use std::str::FromStr;
use syn::parse::{Parse, ParseStream, Result};
use syn::{Expr, ExprCall, ExprMethodCall, Ident};

pub fn handle_call(expr: &ExprCall, var: &Ident) -> TokenStream {
    let ExprCall { func, args, .. } = expr;
    println!(
        "\t\t********\n\nHandling call: {:?}\n\n\t\t********\n\n",
        func
    );
    let func = handle_expr(&func, var);
    let mut grad = quote! { 0.0 };
    for arg in args {
        let arg = handle_expr(&arg, var);
        grad = quote! { #grad + #arg };
    }
    quote! { #func + #grad }
}

pub fn handle_method(expr: &ExprMethodCall, var: &Ident) -> TokenStream {
    let ExprMethodCall {
        args,
        method,
        receiver,
        ..
    } = expr;
    let method_name = method.clone().to_string();

    match UnaryMethod::from_str(&method_name) {
        Ok(method) => handle_unary_method(&method, &receiver),
        Err(_) => {
            let dr = handle_expr(&receiver, var);
            let mut da = quote! { 0.0 };
            for arg in args {
                let arg = handle_expr(&arg, var);
                da = quote! { #da + #arg };
            }
            quote! { #dr + #da }
        }
    }
}

pub fn handle_unary_method(method: &UnaryMethod, recv: &Expr) -> TokenStream {
    match method {
        UnaryMethod::Abs => quote! { #recv / #recv.abs() },
        UnaryMethod::Cos => quote! { -#recv.sin() },
        UnaryMethod::Cosh => quote! { #recv.sinh() },
        UnaryMethod::Exp => {
            quote! {
                if #recv.is_sign_negative() {
                    -#recv.exp()
                } else {
                    #recv.exp()
                }
            }
        }
        UnaryMethod::Inverse | UnaryMethod::Recip => quote! { -#recv.powi(-2) },
        UnaryMethod::Ln => quote! { 1.0 / #recv },
        UnaryMethod::Sin => quote! { #recv.cos() },
        UnaryMethod::Sinh => quote! { #recv.cosh() },
        UnaryMethod::Sqrt => quote! { 1.0 / (2.0 * #recv.sqrt()) },
        UnaryMethod::Tan => quote! { 1.0 / #recv.cos().powi(2) },
        UnaryMethod::Tanh => quote! { 1.0 / #recv.cosh().powi(2) },
    }
}

pub enum Methods {
    Unary(UnaryMethod),
}

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum UnaryMethod {
    Abs,
    Cos,
    Cosh,
    Exp,
    Inverse,
    Ln,
    Recip,
    Sin,
    Sinh,
    Sqrt,
    Tan,
    Tanh,
}

impl FromStr for UnaryMethod {
    type Err = Box<dyn std::error::Error>;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match s {
            "abs" => Ok(UnaryMethod::Abs),
            "cos" | "cosine" => Ok(UnaryMethod::Cos),
            "cosh" => Ok(UnaryMethod::Cosh),
            "exp" => Ok(UnaryMethod::Exp),
            "inv" | "inverse" => Ok(UnaryMethod::Inverse),
            "ln" => Ok(UnaryMethod::Ln),
            "recip" => Ok(UnaryMethod::Recip),
            "sin" | "sine" => Ok(UnaryMethod::Sin),
            "sinh" => Ok(UnaryMethod::Sinh),
            "sqrt" | "square_root" => Ok(UnaryMethod::Sqrt),
            "tan" | "tangent" => Ok(UnaryMethod::Tan),
            "tanh" => Ok(UnaryMethod::Tanh),
            _ => Err("Method not found".into()),
        }
    }
}

pub enum UnaryOps {
    Cosine(kw::cos),
    Exp(kw::e),
    Ln(kw::ln),
    Sine(kw::sin),
    Tan(kw::tan),
    Std(syn::UnOp),
}

impl Parse for UnaryOps {
    fn parse(input: ParseStream) -> Result<Self> {
        if input.peek(syn::Token![.]) {
            if input.peek2(kw::cos) {
                input.parse::<kw::cos>().map(UnaryOps::Cosine)
            } else if input.peek2(kw::sin) {
                input.parse::<kw::sin>().map(UnaryOps::Sine)
            } else if input.peek2(kw::tan) {
                input.parse::<kw::tan>().map(UnaryOps::Tan)
            } else if input.peek2(kw::ln) {
                input.parse::<kw::ln>().map(UnaryOps::Ln)
            } else {
                Err(input.error("Expected a method call"))
            }
        } else {
            input.parse::<syn::UnOp>().map(UnaryOps::Std)
        }
    }
}
