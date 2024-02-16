/*
    Appellation: partials <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
use syn::parse::{Parse, ParseStream, Result};
use syn::{Block, Expr, Ident, Token};

pub struct PartialAst {
    pub expr: Expr,
    pub variable: Ident,
}

impl Parse for PartialAst {
    fn parse(input: ParseStream) -> Result<Self> {
        let variable = input.parse()?;
        input.parse::<Token![:]>()?;
        let expr = input.parse()?;
        Ok(PartialAst { expr, variable })
    }
}

pub struct Partial {
    pub expr: Expr,
    pub var: Ident,
}

impl Parse for Partial {
    fn parse(input: ParseStream) -> Result<Self> {
        let var = input.parse()?;
        input.parse::<Token![:]>()?;
        let expr = input.parse()?;
        Ok(Partial { expr, var })
    }
}
