/*
    Appellation: unary <mod>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
use crate::handle::handle_expr;
use crate::Error;
use proc_macro2::TokenStream;
use quote::quote;
use std::str::FromStr;
use syn::parse::{Parse, ParseStream, Result as ParseResult};
use syn::{BinOp, Expr, Ident, Token};

/// Differentiable binary operations; typically used for defining
/// valid method calls.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[non_exhaustive]
#[repr(u8)]
pub enum BinaryOp {
    Add,
    Div,
    Log,
    Mul,
    Pow,
    Rem,
    Sub,
}

impl BinaryOp {
    pub fn from_binary(op: BinOp) -> crate::Result<Self> {
        use BinOp::*;
        match op {
            Add(_) | AddAssign(_) => Ok(Self::Add),
            Div(_) | DivAssign(_) => Ok(Self::Div),
            Mul(_) | MulAssign(_) => Ok(Self::Mul),
            Rem(_) | RemAssign(_) => Ok(Self::Rem),
            Sub(_) | SubAssign(_) => Ok(Self::Sub),
            _ => Err("Unsupported binary operation".into()),
        }
    }

    pub fn grad(&self, lhs: &Expr, rhs: &Expr, var: &Ident) -> TokenStream {
        // compute the gradient of the left and right hand sides
        let dl = handle_expr(lhs, var);
        let dr = handle_expr(rhs, var);
        // handle various binary operations; returning the gradient
        match *self {
            BinaryOp::Add => quote! { #dl + #dr },
            BinaryOp::Div => quote! { (#dl * #rhs - #lhs * #dr) / (#rhs * #rhs) },
            BinaryOp::Mul => quote! { #dl * #rhs + #lhs * #dr },
            BinaryOp::Pow => {
                quote! {
                    #rhs * #lhs.powf(#rhs - 1.0) * #dl + #lhs.pow(#rhs) * #rhs.ln() * #dr
                }
            }
            BinaryOp::Sub => quote! { #dl - #dr },
            _ => panic!("Unsupported binary method"),
        }
    }
}

impl core::fmt::Display for BinaryOp {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self {
            Self::Add => write!(f, "add"),
            Self::Div => write!(f, "div"),
            Self::Log => write!(f, "log"),
            Self::Mul => write!(f, "mul"),
            Self::Pow => write!(f, "pow"),
            Self::Rem => write!(f, "rem"),
            Self::Sub => write!(f, "sub"),
        }
    }
}

impl FromStr for BinaryOp {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "add" => Ok(Self::Add),
            "div" => Ok(Self::Div),
            "log" => Ok(Self::Log),
            "mul" => Ok(Self::Mul),
            "pow" | "powc" | "powf" | "powi" => Ok(Self::Pow),
            "rem" => Ok(Self::Rem),
            "sub" => Ok(Self::Sub),
            _ => Err("Method not found".into()),
        }
    }
}

impl Parse for BinaryOp {
    fn parse(input: ParseStream) -> ParseResult<Self> {
        if input.peek(Token![.]) {
            if input.peek2(Ident) {
                let method = input.parse::<Ident>()?;
                if let Ok(method) = Self::from_str(method.to_string().as_str()) {
                    return Ok(method);
                }
            }
        } else if input.peek(Token![:]) && input.peek2(Token![:]) {
            let method = input.parse::<Ident>()?;
            if let Ok(method) = Self::from_str(method.to_string().as_str()) {
                return Ok(method);
            }
        }

        Err(input.error("Expected a method call"))
    }
}

impl TryFrom<BinOp> for BinaryOp {
    type Error = Error;

    fn try_from(op: BinOp) -> Result<Self, Self::Error> {
        Self::from_binary(op)
    }
}

impl TryFrom<Ident> for BinaryOp {
    type Error = Error;

    fn try_from(ident: Ident) -> Result<Self, Self::Error> {
        Self::from_str(ident.to_string().as_str())
    }
}
