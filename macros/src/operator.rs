/*
    Appellation: operator <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
//! An attribute macro
//!
//!
use proc_macro2::TokenStream;
use quote::{format_ident, quote, ToTokens};
use syn::{Item, ItemFn, Signature};

pub fn impl_operator(item: Item) -> TokenStream {
    match item {
        Item::Fn(inner) => handle_operator_func(&inner),
        _ => panic!("Expected a function"),
    }
}

pub fn handle_operator_func(item: &ItemFn) -> TokenStream {
    let item_tk = item.to_token_stream();
    let item_str = item_tk.to_string();
    let ItemFn { sig, .. } = item;
    let Signature { ident, .. } = sig;

    let lexical = format_ident!("{}_lexical", ident);
    let lex_const = format_ident!("{}", lexical.to_string().to_uppercase());
    quote! {
        #item

        pub const #lex_const: &str = #item_str;

        pub fn #lexical() -> String {
            #item_str.to_string()
        }


    }
}
