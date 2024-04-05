/*
    Appellation: unary <mod>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
use std::str::FromStr;
use syn::parse::{Parse, ParseStream, Result as ParseResult};

/// Additional unary methods that may be handled
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[non_exhaustive]
pub enum UnaryOp {
    Abs,
    Cos,
    Cosh,
    Exp,
    Inverse,
    Ln,
    Recip,
    Sin,
    Sinh,
    Sqrt,
    Tan,
    Tanh,
}

impl FromStr for UnaryOp {
    type Err = Box<dyn std::error::Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "abs" => Ok(UnaryOp::Abs),
            "cos" | "cosine" => Ok(UnaryOp::Cos),
            "cosh" => Ok(UnaryOp::Cosh),
            "exp" => Ok(UnaryOp::Exp),
            "inv" | "inverse" => Ok(UnaryOp::Inverse),
            "ln" => Ok(UnaryOp::Ln),
            "recip" => Ok(UnaryOp::Recip),
            "sin" | "sine" => Ok(UnaryOp::Sin),
            "sinh" => Ok(UnaryOp::Sinh),
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
