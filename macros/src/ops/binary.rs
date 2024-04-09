/*
    Appellation: unary <mod>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
use crate::BoxError;
use std::str::FromStr;
use syn::parse::{Parse, ParseStream, Result as ParseResult};
use syn::{Ident, Token};

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
    Sub,
}

impl core::fmt::Display for BinaryOp {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self {
            Self::Add => write!(f, "add"),
            Self::Div => write!(f, "div"),
            Self::Log => write!(f, "log"),
            Self::Mul => write!(f, "mul"),
            Self::Pow => write!(f, "pow"),
            Self::Sub => write!(f, "sub"),
        }
    }
}

impl FromStr for BinaryOp {
    type Err = BoxError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "add" => Ok(Self::Add),
            "div" => Ok(Self::Div),
            "log" => Ok(Self::Log),
            "mul" => Ok(Self::Mul),
            "pow" | "powc" | "powf" | "powi" => Ok(Self::Pow),
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
        } else if input.peek(Token![:]) {
            if input.peek2(Token![:]) {
                let method = input.parse::<Ident>()?;
                if let Ok(method) = Self::from_str(method.to_string().as_str()) {
                    return Ok(method);
                }
            }
        }

        Err(input.error("Expected a method call"))
    }
}
