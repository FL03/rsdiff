/*
    Appellation: item <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
use super::block::handle_block;
use proc_macro2::TokenStream;
use syn::{Ident, Item, ItemFn, Signature};

pub fn handle_item(item: &Item, var: &Ident) -> TokenStream {
    match item {
        Item::Fn(inner) => handle_item_fn(inner, var),

        _ => panic!("Unsupported item!"),
    }
}

pub fn handle_item_fn(item: &ItemFn, var: &Ident) -> TokenStream {
    let ItemFn { block, sig, .. } = item;
    let Signature { inputs, .. } = sig;

    let mut vars = Vec::new();
    for input in inputs {
        if let syn::FnArg::Typed(typed) = input {
            if let syn::Pat::Ident(ident) = &*typed.pat {
                vars.push(ident.ident.clone());
            }
        }
    }

    handle_block(&block, &var)
}
