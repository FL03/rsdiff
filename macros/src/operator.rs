/*
    Appellation: operator <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
//! An attribute macro
//!
//!
use proc_macro2::TokenStream;
use quote::{format_ident, quote, ToTokens};
use syn::{Expr, FnArg, Ident, Item, ItemFn, Signature};
use syn::spanned::Spanned;

pub fn impl_operator(item: Item) -> TokenStream {
    match item {
        Item::Fn(inner) => handle_operator_func(&inner),
        _ => panic!("Expected a function"),
    }
}

fn fn_args_ident(arg: &FnArg) -> Ident {
    use syn::Pat;
    match arg {
        FnArg::Typed(inner) => match inner.pat.as_ref() {
            Pat::Ident(ident) => {
                ident.ident.clone()
            }
            _ => panic!("Expected an identifier"),
        }
        _ => panic!("Expected a typed argument"),
    }
}

fn handle_operator_func(item: &ItemFn) -> TokenStream {
    let item_tk = item.to_token_stream();
    let item_str = item_tk.to_string();
    let _lit = {
        let lit_str = syn::LitStr::new(&item_str, item.span());
        syn::Lit::Str(lit_str)
    };
    // let grad: Vec<TokenStream> = item.sig.inputs.iter().map(|i| crate::handle::handle_expr(&Expr::Lit(syn::ExprLit {attrs: Vec::new(), lit: _lit}), &fn_args_ident(i))).collect();
    let ItemFn { sig, .. } = item;
    let Signature { ident, .. } = sig;
    let ident_grad = format_ident!("{}_grad", ident);
    let lexical = format_ident!("{}_lexical", ident);
    let lex_const = format_ident!("{}", lexical.to_string().to_uppercase());
    quote! {
        #item

        pub const #lex_const: &str = #item_str;

        macro_rules! #ident_grad {
            () => {
                #item
            };
        }

        pub fn #lexical() -> String {
            #item_str.to_string()
        }


    }
}
