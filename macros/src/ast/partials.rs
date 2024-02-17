/*
    Appellation: partials <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
use syn::parse::{Parse, ParseStream, Result};
use syn::punctuated::Punctuated;
use syn::{Expr, ExprCall, ExprMethodCall, Ident, ItemFn, Token, Type};

mod kw {

    syn::custom_keyword!(ExprMethodCall);
    syn::custom_keyword!(partial);
    syn::custom_keyword!(partials);
}
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

pub enum PartialFn {
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
            Err(input.error("Expected a function call or method call"))
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