/*
    Appellation: autodiff <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
use super::handle::expr::handle_expr;
use super::handle::item::handle_item;
use crate::ast::partials::*;
use proc_macro2::TokenStream;
use syn::Ident;

pub fn generate_autodiff(partial: &PartialAst) -> TokenStream {
    let PartialAst { expr, var, .. } = partial;
    let grad = handle_input(&expr, &var);
    grad
}

fn handle_input(input: &PartialFn, var: &Ident) -> TokenStream {
    match input {
        PartialFn::Expr(inner) => handle_expr(&inner, var),
        PartialFn::Item(inner) => handle_item(&inner.clone().into(), var),
    }
}
