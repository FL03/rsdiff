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

#[allow(dead_code)]
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
