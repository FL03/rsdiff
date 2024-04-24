/*
    Appellation: gradient <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
#![allow(dead_code)]
use proc_macro2::TokenStream;
use syn::parse::{Parse, ParseStream, Result};
use syn::{Attribute, ExprArray, ItemFn};

pub struct GradientAst {
    pub attrs: Vec<Attribute>,
    pub item: ItemFn,
}

impl GradientAst {
    pub fn new(attrs: Vec<Attribute>, item: ItemFn) -> Self {
        Self { attrs, item }
    }

    pub fn attributes(&self) -> &[Attribute] {
        &self.attrs
    }

    pub fn item(&self) -> &ItemFn {
        &self.item
    }
}

impl Parse for GradientAst {
    fn parse(input: ParseStream) -> Result<Self> {
        let attrs = input.call(Attribute::parse_outer)?;
        let item = input.parse()?;
        Ok(GradientAst { attrs, item })
    }
}

pub enum ExprGrad {
    Array(ExprArray),
    Verbatim(TokenStream),
}

impl Parse for ExprGrad {
    fn parse(input: ParseStream) -> Result<Self> {
        if let Ok(arr) = input.parse::<ExprArray>() {
            return Ok(ExprGrad::Array(arr));
        }
        Ok(ExprGrad::Verbatim(input.parse()?))
    }
}

impl From<ExprArray> for ExprGrad {
    fn from(arr: ExprArray) -> Self {
        ExprGrad::Array(arr)
    }
}

impl From<TokenStream> for ExprGrad {
    fn from(stream: TokenStream) -> Self {
        ExprGrad::Verbatim(stream)
    }
}
