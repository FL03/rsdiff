/*
    Appellation: partials <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
use proc_macro2::TokenStream;
use syn::parse::{Parse, ParseStream, Result};
use syn::{Attribute, Item, ItemFn};

pub enum PartialScope {
    Fn(ItemFn),
    Verbatim(TokenStream), // Not considered
}

impl Parse for PartialScope {
    fn parse(input: ParseStream) -> Result<Self> {
        let item: Item = input.parse()?;
        match item {
            Item::Fn(item_fn) => Ok(Self::Fn(item_fn)),
            _ => {
                dbg!("Not handled by acme yet");
                Ok(Self::Verbatim(input.parse()?))
            }
        }
    }
}

pub struct PartialAst {
    pub attrs: Vec<Attribute>, // #[partial(attr, ..)]
    // pub args: Punctuated<Type, Token![,]>, // x1: T1, ..., xn: Tn
    pub scope: PartialScope,
}

impl Parse for PartialAst {
    fn parse(input: ParseStream) -> Result<Self> {
        let attrs = input.call(Attribute::parse_outer)?;
        let scope = input.parse()?;
        Ok(Self { attrs, scope })
    }
}
