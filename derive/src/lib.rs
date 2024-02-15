/*
    Appellation: acme-derive <library>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
//! # acme-derive
//!
//!
extern crate proc_macro;
use proc_macro::TokenStream;

#[proc_macro_derive(AnswerFn)]
pub fn derive_answer_fn(_item: TokenStream) -> TokenStream {
    "fn answer() -> u32 { 42 }".parse().unwrap()
}

#[proc_macro_derive(HelperAttr, attributes(helper))]
pub fn derive_helper_attr(_item: TokenStream) -> TokenStream {
    TokenStream::new()
}

#[proc_macro_derive(Params, attributes(param))]
pub fn derive_params(_item: TokenStream) -> TokenStream {
    TokenStream::new()
}