/*
    Appellation: ad <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
use crate::ast::partials::{PartialAst, PartialFn};
use crate::handle::{expr, item};
use proc_macro2::TokenStream;

pub fn impl_autodiff(partial: &PartialAst) -> TokenStream {
    let PartialAst { expr, var, .. } = partial;

    match expr {
        PartialFn::Expr(inner) => expr::handle_expr(inner, var),
        PartialFn::Item(inner) => item::handle_item(&inner.clone().into(), var),
    }
}
