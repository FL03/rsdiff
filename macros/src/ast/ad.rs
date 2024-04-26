/*
    Appellation: ad <ast>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
use crate::handle::{expr, item};
use proc_macro2::TokenStream;
use syn::parse::{Parse, ParseStream, Result};
use syn::{Expr, Ident, ItemFn, Token};

// #77 Try to integrate with the #[operator] macro by collecting the String created by invoking <call>_lexical()
pub enum Scope {
    Expr(Expr),
    Item(ItemFn),
    Verbatim(TokenStream), // Not considered
}

impl Scope {
    pub fn handle(&self, args: &Ident) -> TokenStream {
        match self {
            Scope::Expr(scope) => expr::handle_expr(scope, args),
            Scope::Item(scope) => item::handle_item_fn(scope, args),
            Scope::Verbatim(_) => panic!("Custom functions not yet supported"),
        }
    }
}

impl Parse for Scope {
    fn parse(input: ParseStream) -> Result<Self> {
        if let Ok(item) = input.parse() {
            Ok(Self::Item(item))
        } else if let Ok(expr) = input.parse() {
            Ok(Self::Expr(expr))
        } else {
            Ok(Scope::Verbatim(input.parse()?))
        }
    }
}

pub struct AutodiffAst {
    pub args: Ident,
    pub split: Token![:],
    pub scope: Scope,
}

impl Parse for AutodiffAst {
    fn parse(input: ParseStream) -> Result<Self> {
        let args = input.parse()?;
        let split = input.parse::<Token![:]>()?;
        let scope = input.parse()?;
        Ok(Self { args, split, scope })
    }
}
