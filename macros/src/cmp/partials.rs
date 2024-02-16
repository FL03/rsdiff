/*
    Appellation: partials <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
use syn::parse::{Parse, ParseStream, Result};
use syn::{Block, Expr, Ident, Token};

pub struct PartialDerivative {
    pub expr: Expr,
    pub variable: Ident,
}

impl Parse for PartialDerivative {
    fn parse(input: ParseStream) -> Result<Self> {
        let variable = input.parse()?;
        input.parse::<Token![:]>()?;
        let expr = input.parse()?;
        Ok(PartialDerivative { expr, variable })
    }
}

pub struct Partial {
    pub block: Block,
    pub var: Ident,
}

impl Parse for Partial {
    fn parse(input: ParseStream) -> Result<Self> {
        let var = input.parse()?;
        input.parse::<Token![:]>()?;
        let block = input.parse()?;
        Ok(Partial { block, var })
    }
}
