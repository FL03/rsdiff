/*
    Appellation: ast <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/

pub mod partials;

use proc_macro2::Span;
use quote::{quote, ToTokens};
use syn::Expr;
use syn::parse::{Parse, ParseStream, Result};
use syn::spanned::Spanned;

pub struct Ast {
    pub expr: Expr,
    pub span: Span,
}

impl Parse for Ast {
    fn parse(input: ParseStream) -> Result<Self> {
        // let span = input.span();

        let expr: Expr = input.parse()?;
        let span = expr.span();
        Ok(Self { expr, span, })
    }
}

impl ToTokens for Ast {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        tokens.extend(quote! {#self.expr })
    }
}