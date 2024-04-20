/*
    Appellation: partials <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
use proc_macro2::TokenStream;
use syn::parse::{Parse, ParseStream, Result};
use syn::punctuated::Punctuated;
use syn::{Attribute, Block, Expr, Ident, ItemFn, Signature, Token, Type};

pub struct Partial {
    pub expr: Expr,
    pub var: Ident,
}

impl Parse for Partial {
    fn parse(input: ParseStream) -> Result<Self> {
        let variable = input.parse()?;
        input.parse::<Token![:]>()?;
        let expr = input.parse()?;
        Ok(Partial {
            expr,
            var: variable,
        })
    }
}

pub struct Partials {
    pub expr: Expr,
    pub split: Token![:],
    pub vars: Punctuated<Type, Token![,]>,
}

impl Parse for Partials {
    fn parse(input: ParseStream) -> Result<Self> {
        let vars = input.parse_terminated(Type::parse, Token![,])?;
        let split = input.parse::<Token![:]>()?;
        let expr = input.parse()?;
        Ok(Self { expr, split, vars })
    }
}

pub struct PartialFnCall {
    pub attrs: Vec<Attribute>,
    pub body: Box<Block>,
    pub sig: Signature,
}

impl Parse for PartialFnCall {
    fn parse(input: ParseStream) -> Result<Self> {
        let attrs = input.call(Attribute::parse_outer)?;
        let sig: Signature = input.parse()?;
        let body: Block = input.parse()?;
        Ok(Self {
            attrs,
            body: Box::new(body),
            sig,
        })
    }
}

pub enum PartialFn {
    Custom(TokenStream),
    Expr(Expr),
    Item(ItemFn),
}

impl Parse for PartialFn {
    fn parse(input: ParseStream) -> Result<Self> {
        if let Ok(item) = input.parse() {
            Ok(Self::Item(item))
        } else if let Ok(expr) = input.parse() {
            Ok(Self::Expr(expr))
        } else {
            Ok(PartialFn::Custom(input.parse()?))
        }
    }
}

pub struct PartialAst {
    pub expr: PartialFn,
    pub split: Token![:],
    pub var: Ident,
}

impl Parse for PartialAst {
    fn parse(input: ParseStream) -> Result<Self> {
        let var = input.parse()?;
        let split = input.parse::<Token![:]>()?;
        let expr = input.parse()?;
        Ok(Self { expr, split, var })
    }
}
