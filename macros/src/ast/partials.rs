/*
    Appellation: partials <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
use syn::parse::{Parse, ParseStream, Result};
use syn::punctuated::Punctuated;
use syn::{Expr, Ident, Token, Type};

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

pub struct PartialAst {
    pub expr: Expr,
    pub split: Token![:],
    pub vars: Punctuated<Type, Token![,]>,
}

impl Parse for PartialAst {
    fn parse(input: ParseStream) -> Result<Self> {
        let vars = input.parse_terminated(Type::parse, Token![,])?;
        let split = input.parse::<Token![:]>()?;
        let expr = input.parse()?;
        Ok(Self { expr, split, vars })
    }
}
