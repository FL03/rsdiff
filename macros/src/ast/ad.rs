/*
    Appellation: ad <ast>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
use proc_macro2::TokenStream;
use syn::parse::{Parse, ParseStream, Result};
use syn::{Expr, Ident, ItemFn, Token};

// #77 Try to integrate with the #[operator] macro by collecting the String created by invoking <call>_lexical()
pub enum PartialFn {
    Expr(Expr),
    Item(ItemFn),
    Verbatim(TokenStream), // Not considered
}

impl Parse for PartialFn {
    fn parse(input: ParseStream) -> Result<Self> {
        if let Ok(item) = input.parse() {
            Ok(Self::Item(item))
        } else if let Ok(expr) = input.parse() {
            Ok(Self::Expr(expr))
        } else {
            Ok(PartialFn::Verbatim(input.parse()?))
        }
    }
}

pub struct AutodiffAst {
    pub scope: PartialFn,
    pub split: Token![:],
    pub var: Ident,
}

impl Parse for AutodiffAst {
    fn parse(input: ParseStream) -> Result<Self> {
        let var = input.parse()?;
        let split = input.parse::<Token![:]>()?;
        let scope = input.parse()?;
        Ok(Self { scope, split, var })
    }
}

pub enum AutodiffVar {
    Ident(Ident),
    Let(syn::ExprLet),
}
