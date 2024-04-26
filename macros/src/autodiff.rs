/*
    Appellation: ad <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
use crate::ast::ad::{AutodiffAst, Scope};
use crate::handle::{expr, item};
use proc_macro2::TokenStream;

pub fn impl_autodiff(partial: &AutodiffAst) -> TokenStream {
    let AutodiffAst {
        scope: expr, var, ..
    } = partial;

    match expr {
        Scope::Expr(inner) => expr::handle_expr(inner, var),
        Scope::Item(inner) => item::handle_item(&inner.clone().into(), var),
        Scope::Verbatim(_inner) => panic!("Custom functions not yet supported"),
    }
}
