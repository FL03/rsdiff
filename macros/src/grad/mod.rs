/*
    Appellation: grad <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/


use crate::ast::gradient::GradientAst;
use crate::diff::handle::block::handle_block;
use quote::quote;
use proc_macro2::TokenStream;
use syn::{ItemFn, Signature};

pub fn gradient(grad: &GradientAst) -> TokenStream {
    let GradientAst { attrs, item } = grad;
    let attrs = attrs;
    let item = item;
    let output = quote! {
        #(#attrs)*
        #item
    };
    output
}

fn handle_item_fn(item: &ItemFn) -> TokenStream {
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

    let grad = vars.iter().map(|var| handle_block(&block, &var)).collect::<Vec<_>>();

    quote! {
        [#(#grad)*]
    }
}