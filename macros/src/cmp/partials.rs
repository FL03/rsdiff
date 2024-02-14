/*
    Appellation: partials <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
use syn::parse::{Parse, ParseStream, Result};
use syn::{Expr, Ident, Token};

pub struct PartialDerivative {
    pub expr: Expr,
    pub variable: Ident,
}

impl Parse for PartialDerivative {
    fn parse(input: ParseStream) -> Result<Self> {
        let variable = input.parse()?;
        input.parse::<Token![:]>()?;
        let expr = input.parse()?;
        input.parse::<Token![;]>()?;
        Ok(PartialDerivative { expr, variable })
    }
}
