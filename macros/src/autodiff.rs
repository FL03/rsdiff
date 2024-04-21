/*
    Appellation: ad <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
use crate::ast::ad::{AutodiffAst, PartialFn};
use crate::handle::{expr, item};
use proc_macro2::TokenStream;

pub fn impl_autodiff(partial: &AutodiffAst) -> TokenStream {
    let AutodiffAst { expr, var, .. } = partial;

    match expr {
        PartialFn::Expr(inner) => expr::handle_expr(inner, var),
        PartialFn::Item(inner) => item::handle_item(&inner.clone().into(), var),
        PartialFn::Verbatim(_inner) => panic!("Custom functions not yet supported"),
    }
}
