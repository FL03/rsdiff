/*
    Appellation: gradient <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
use syn::parse::{Parse, ParseStream, Result};
use syn::{Attribute, ItemFn};

pub struct GradientAst {
    pub attrs: Vec<Attribute>,
    pub item: ItemFn,
}

impl Parse for GradientAst {
    fn parse(input: ParseStream) -> Result<Self> {
        let attrs = input.call(Attribute::parse_outer)?;
        let item = input.parse()?;
        Ok(GradientAst { attrs, item })
    }
}
