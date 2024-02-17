/*
    Appellation: item <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
use super::block::handle_block;
use proc_macro2::TokenStream;
use syn::{Ident, Item, ItemFn};

pub fn handle_item(item: &Item, var: &Ident) -> TokenStream {
    match item {
        Item::Fn(inner) => {
            let ItemFn { block, .. } = inner;
            handle_block(&block, var)
        }

        _ => panic!("Unsupported item!"),
    }
}