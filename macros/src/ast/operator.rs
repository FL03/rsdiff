/*
    Appellation: operator <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
use syn::{Attribute, ItemFn};

pub struct OperatorAst {
    pub attrs: Vec<Attribute>,
    pub item: ItemFn,
}

impl OperatorAst {
    pub fn new(attrs: Vec<Attribute>, item: ItemFn) -> Self {
        Self { attrs, item }
    }
    #[allow(dead_code)]
    pub fn from_pma(attr: proc_macro::TokenStream, item: proc_macro::TokenStream) -> Self {
        let attrs = syn::parse_macro_input!(attr with Attribute::parse_outer);
        let item = syn::parse_macro_input!(item as ItemFn);
        Self::new(attrs, item)
    }
}