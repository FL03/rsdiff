/*
    Appellation: ad <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
use crate::ast::ad::{AutodiffAst, Scope};
use crate::handle::{expr, item};
use proc_macro2::TokenStream;

pub fn impl_autodiff(partial: &AutodiffAst) -> TokenStream {
    let AutodiffAst { args, scope, .. } = partial;

    match scope {
        Scope::Expr(inner) => expr::handle_expr(inner, args),
        Scope::Item(inner) => item::handle_item(&inner.clone().into(), args),
        Scope::Verbatim(_inner) => panic!("Custom functions not yet supported"),
    }
}
