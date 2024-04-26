/*
    Appellation: ast <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
#![allow(dead_code)]
pub use self::{ad::AutodiffAst, operator::OperatorAst};

pub mod ad;
pub mod grad;
pub mod operator;

use proc_macro2::{Span, TokenStream};
use quote::{quote, ToTokens};
use syn::parse::{Parse, ParseStream, Result};
use syn::Expr;

pub struct BackendAst {
    pub args: Args,
    pub span: Span,
}

impl Parse for BackendAst {
    fn parse(input: ParseStream) -> Result<Self> {
        let args = input.parse()?;
        let span = Span::call_site();
        Ok(Self { args, span })
    }
}

impl ToTokens for BackendAst {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        tokens.extend(quote! {#self.expr })
    }
}

pub enum Args {
    Block(syn::Block),
    Expr(Expr),
    Item(syn::Item),
    Verbatim(TokenStream),
}

impl Parse for Args {
    fn parse(input: ParseStream) -> Result<Self> {
        if let Ok(block) = input.parse() {
            Ok(Args::Block(block))
        } else if let Ok(item) = input.parse() {
            Ok(Args::Item(item))
        } else if let Ok(expr) = input.parse() {
            Ok(Args::Expr(expr))
        } else {
            dbg!("Currently not handled");
            Ok(Args::Verbatim(input.parse()?))
        }
    }
}
