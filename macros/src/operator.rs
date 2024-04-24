/*
    Appellation: operator <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
//! An attribute macro
//!
//!
use crate::ast::operator::{OperatorAst, OperatorAttr};
use proc_macro2::TokenStream;
use quote::{format_ident, quote, ToTokens};
use syn::spanned::Spanned;
use syn::{Item, ItemFn, Lit, LitStr, Signature};

pub fn impl_operator(ast: &OperatorAst) -> TokenStream {
    let OperatorAst { attrs, item, .. } = ast;
    let mut res = match item {
        Item::Fn(inner) => handle_operator_func(&inner),
        _ => panic!("Expected a function"),
    };
    
    if let Some(attrs) = attrs {
        let ext = handle_operator_attr(&attrs, &item);
        res = quote! {
            #res

            #ext
        };
    }
    res
}

fn handle_operator_attr(attrs: &OperatorAttr, item: &Item) -> TokenStream {
    let OperatorAttr { lexical } = attrs;
    let item_tk = item.to_token_stream();
    let item_str = item_tk.to_string();
    let mut res = TokenStream::new();
    if let Some(lex) = lexical {
        let lex_const = format_ident!("{}", lex.to_string().to_uppercase());
        let lex_func = format_ident!("{}", lex.to_string().to_lowercase());
        res = quote! {
            #res

            pub const #lex_const: &str = #item_str;

            pub fn #lex_func() -> String {
                #item_str.to_string()
            }
        }
    }
    res
}

fn handle_operator_func(item: &ItemFn) -> TokenStream {
    let item_tk = item.to_token_stream();
    let item_str = item_tk.to_string();
    let _lit = {
        let lit_str = LitStr::new(&item_str, item.span());
        Lit::Str(lit_str)
    };
    let Signature { ident, .. } = &item.sig;
    let ident_grad = format_ident!("{}_grad", ident);
    quote! {
        #item

        macro_rules! #ident_grad {
            () => {
                #item
            };
        }

    }
}
