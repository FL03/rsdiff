/*
    Appellation: unary <mod>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
use crate::handle::expr::handle_expr;
use proc_macro2::TokenStream;
use quote::quote;
use std::str::FromStr;
use syn::parse::{Parse, ParseStream, Result as ParseResult};
use syn::{Expr, Ident};

/// Additional unary methods that may be handled
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[non_exhaustive]
pub enum UnaryOp {
    Abs,
    Cos,
    Cosh,
    Cubed,
    CubeRoot,
    Exp,
    Inverse,
    Ln,
    Recip,
    Sin,
    Sinh,
    Square,
    Sqrt,
    Tan,
    Tanh,
}

impl UnaryOp {
    pub fn grad(&self, recv: &Expr, wrt: &Ident) -> TokenStream {
        let grad = handle_expr(recv, wrt);

        let ds = match *self {
            UnaryOp::Abs => quote! { #recv / #recv.abs() },
            UnaryOp::Cos => quote! { -#recv.sin() },
            UnaryOp::Cosh => quote! { #recv.sinh() },
            UnaryOp::Cubed => quote! { 3.0 * #recv.pow(2) },
            UnaryOp::CubeRoot => quote! { #recv.powi(-2) / 3.0 },
            UnaryOp::Exp => {
                quote! {
                    if #recv.is_sign_negative() {
                        -#recv.exp()
                    } else {
                        #recv.exp()
                    }
                }
            }
            UnaryOp::Inverse | UnaryOp::Recip => quote! { -#recv.powi(-2) },
            UnaryOp::Ln => quote! { #recv.recip() },
            UnaryOp::Sin => quote! { #recv.cos() },
            UnaryOp::Sinh => quote! { #recv.cosh() },
            UnaryOp::Square => quote! { 2.0 * #recv },
            UnaryOp::Sqrt => quote! { (2.0 * #recv.sqrt()).recip() },
            UnaryOp::Tan => quote! { #recv.cos().powi(2).recip() },
            UnaryOp::Tanh => quote! { #recv.cosh().powi(2).recip() },
        };
        quote! { #ds * #grad }
    }
}

impl FromStr for UnaryOp {
    type Err = Box<dyn std::error::Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "abs" => Ok(UnaryOp::Abs),
            "cos" | "cosine" => Ok(UnaryOp::Cos),
            "cosh" => Ok(UnaryOp::Cosh),
            "cubed" => Ok(UnaryOp::Cubed),
            "cbr" | "cube_root" => Ok(UnaryOp::CubeRoot),
            "exp" => Ok(UnaryOp::Exp),
            "inv" | "inverse" => Ok(UnaryOp::Inverse),
            "ln" => Ok(UnaryOp::Ln),
            "recip" => Ok(UnaryOp::Recip),
            "sin" | "sine" => Ok(UnaryOp::Sin),
            "sinh" => Ok(UnaryOp::Sinh),
            "sqr" | "square" => Ok(UnaryOp::Square),
            "sqrt" | "square_root" => Ok(UnaryOp::Sqrt),
            "tan" | "tangent" => Ok(UnaryOp::Tan),
            "tanh" => Ok(UnaryOp::Tanh),
            _ => Err("Method not found".into()),
        }
    }
}

impl Parse for UnaryOp {
    fn parse(input: ParseStream) -> ParseResult<Self> {
        if input.peek(syn::Token![.]) {
            if input.peek2(syn::Ident) {
                let method = input.parse::<syn::Ident>()?;
                if let Ok(method) = UnaryOp::from_str(method.to_string().as_str()) {
                    return Ok(method);
                }
            }
        }
        Err(input.error("Expected a method call"))
    }
}
