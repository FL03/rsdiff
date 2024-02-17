/*
    Appellation: autodiff <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
use crate::ast::partials::*;
use crate::partial::handle_expr as handle;
use proc_macro2::{Span, TokenStream};
use quote::quote;
use syn::parse::{Parse, ParseStream};
use syn::{Block, Expr, ExprCall, ExprClosure, ExprMethodCall, Ident, ItemFn, Result, Stmt};

mod kw {
    syn::custom_keyword!(autodiff);
    syn::custom_keyword!(ln);
}

pub fn generate_autodiff(partial: &PartialAst) -> TokenStream {
    let PartialAst { expr, var, .. } = partial;
    let grad = handle_input(&expr, &var);
    grad
}

fn handle_input(input: &PartialFn, var: &Ident) -> TokenStream {
    match input {
        PartialFn::Expr(inner) => handle_expr(&inner, var),
        PartialFn::Item(inner) => handle_item(&inner, var),
    }
}

fn handle_item(item: &ItemFn, var: &Ident) -> TokenStream {
    let ItemFn { block, .. } = item;
    handle_block(&block, var)
}

fn handle_block(block: &Block, var: &Ident) -> TokenStream {
    let Block { stmts, .. } = block;
    let mut grad = quote! { 0.0 };
    for stmt in stmts {
        let stmt = match stmt {
            Stmt::Expr(expr, _semi) => handle_expr(&expr, var),
            _ => panic!("Unsupported statement!"),
        };
        grad = quote! { #grad + #stmt };
    }
    grad
}

fn handle_expr(expr: &Expr, var: &Ident) -> TokenStream {
    match expr {
        Expr::Call(inner) => handle_call(inner, var),
        Expr::Closure(inner) => handle_closure(inner, var),
        Expr::MethodCall(inner) => handle_method(inner, var),
        _ => handle(expr, var),
    }
}

fn handle_call(expr: &ExprCall, var: &Ident) -> TokenStream {
    let ExprCall { func, args, .. } = expr;
    let func = handle_expr(&func, var);
    let mut grad = quote! { 0.0 };
    for arg in args {
        let arg = handle_expr(&arg, var);
        grad = quote! { #grad + #arg };
    }

    grad
}

fn handle_closure(expr: &ExprClosure, var: &Ident) -> TokenStream {
    let ExprClosure { body, .. } = expr;
    handle_expr(body, var)
}

fn handle_method(expr: &ExprMethodCall, var: &Ident) -> TokenStream {
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

    quote! { #dr + #da }
}

pub enum Ops {
    Cosine,
    Sine,
    Tan,
    Unary(UnaryOps),
}

pub enum UnaryOps {
    Ln(kw::ln),
    Std(syn::UnOp),
}

impl Parse for UnaryOps {
    fn parse(input: ParseStream) -> Result<Self> {
        if input.peek(kw::ln) {
            input.parse::<kw::ln>().map(UnaryOps::Ln)
        } else {
            input.parse::<syn::UnOp>().map(UnaryOps::Std)
        }
    }
}
