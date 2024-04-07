/*
    Appellation: unary <mod>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
use std::str::FromStr;
use syn::parse::{Parse, ParseStream, Result as ParseResult};
use syn::{Ident, Token};

pub type BoxError = Box<dyn std::error::Error>;
/// Additional unary methods that may be handled
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[non_exhaustive]
pub enum BinaryOp {
    Pow,
    
}

impl core::fmt::Display for BinaryOp {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self {
            Self::Pow => write!(f, "pow"),
        }
    }
}

impl FromStr for BinaryOp {
    type Err = BoxError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "pow" | "powc" | "powf" | "powi" => Ok(Self::Pow),
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
