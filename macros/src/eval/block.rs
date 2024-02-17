/*
    Appellation: block <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
use super::stmt::handle_stmt;
use proc_macro2::TokenStream;
use quote::quote;
use syn::{Block, Ident};

pub fn handle_block(block: &Block, var: &Ident) -> TokenStream {
    let Block { stmts, .. } = block;
    let mut grad = quote! { 0.0 };
    for stmt in stmts {
        let stmt = handle_stmt(stmt, var);
        grad = quote! { #grad + #stmt };
    }
    grad
}