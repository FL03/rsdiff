/*
    Appellation: method <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
use super::handle_expr;
use crate::kw;
use proc_macro2::TokenStream;
use quote::quote;
use syn::{ExprCall, ExprMethodCall, Ident};
use syn::parse::{Parse, ParseStream, Result};

pub fn handle_call(expr: &ExprCall, var: &Ident) -> TokenStream {
    let ExprCall { func, args, .. } = expr;
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
        receiver,
        args,
        method,
        ..
    } = expr;
    let method_name = method.clone().to_string();
    let dr = handle_expr(&receiver, var);
    let mut da = quote! { 0.0 };
    for arg in args {
        let arg = handle_expr(&arg, var);
        da = quote! { #da + #arg };
    }

    if method_name == "ln" {
        return quote! { 1.0 / #receiver };
    }
    if method_name == "cos" {
        return quote! { -#receiver.sin() };
    }
    if method_name == "sin" {
        return quote! { #receiver.cos() };
    }
    if method_name == "tan" {
        return quote! { 1.0 / #receiver.cos().powi(2)};
    }

    quote! { #dr + #da }
}



pub enum Ops {

    Unary(UnaryOps),
}

pub enum UnaryOps {
    Cosine(kw::cos),
    Sine(kw::sin),
    Tan(kw::tan),
    Ln(kw::ln),
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
