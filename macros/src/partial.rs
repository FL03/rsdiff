/*
    Appellation: partial <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
//! An attribute macro
//!
//!
use crate::ast::grad::GradientAst;
use crate::handle::block::handle_block;
use proc_macro2::TokenStream;
use quote::quote;
use syn::{ItemFn, Signature};

#[allow(dead_code)]
pub fn partial_impl(grad: &GradientAst) -> TokenStream {
    let GradientAst { attrs, item } = grad;
    let _attrs = attrs;
    let item = item;
    handle_item_fn(&item)
}

pub fn handle_item_fn(item: &ItemFn) -> TokenStream {
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

    let grad = vars
        .iter()
        .map(|var| handle_block(&block, &var))
        .collect::<Vec<_>>();

    quote! {
        [#(#grad)*]
    }
}

#[allow(dead_code)]
pub fn item_fn_partial(item: &ItemFn) -> TokenStream {
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

    let grad = vars
        .iter()
        .map(|var| handle_block(&block, &var))
        .collect::<Vec<_>>();

    quote! {
        [#(#grad)*]
    }
}
