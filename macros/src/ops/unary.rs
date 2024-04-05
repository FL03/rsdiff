use crate::kw;
use proc_macro2::Span;
use std::str::FromStr;
use syn::parse::{Parse, ParseStream, Result};

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum UnaryMethod {
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

impl FromStr for UnaryMethod {
    type Err = Box<dyn std::error::Error>;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match s {
            "abs" => Ok(UnaryMethod::Abs),
            "cos" | "cosine" => Ok(UnaryMethod::Cos),
            "cosh" => Ok(UnaryMethod::Cosh),
            "exp" => Ok(UnaryMethod::Exp),
            "inv" | "inverse" => Ok(UnaryMethod::Inverse),
            "ln" => Ok(UnaryMethod::Ln),
            "recip" => Ok(UnaryMethod::Recip),
            "sin" | "sine" => Ok(UnaryMethod::Sin),
            "sinh" => Ok(UnaryMethod::Sinh),
            "sqrt" | "square_root" => Ok(UnaryMethod::Sqrt),
            "tan" | "tangent" => Ok(UnaryMethod::Tan),
            "tanh" => Ok(UnaryMethod::Tanh),
            _ => Err("Method not found".into()),
        }
    }
}

impl Parse for UnaryMethod {
    fn parse(input: ParseStream) -> Result<Self> {
        if input.peek(syn::Token![.]) {
            if input.peek2(syn::Ident) {
                let method = input.parse::<syn::Ident>()?;
                if let Ok(method) = UnaryMethod::from_str(method.to_string().as_str()) {
                    return Ok(method);
                }
            }
        }
        Err(input.error("Expected a method call"))
    }
}

pub enum UnaryOps {
    Cosine(kw::cos),
    Exp(kw::exp),
    Ln(kw::ln),
    Sine(kw::sin),
    Tan(kw::tan),
    Std(syn::UnOp),
}

impl Parse for UnaryOps {
    fn parse(input: ParseStream) -> Result<Self> {
        if input.peek(syn::Token![.]) {
            if input.peek2(syn::Ident) {
                let method = input.parse::<syn::Ident>()?;
                let span = Span::call_site();
                match method.to_string().as_str() {
                    "cos" => return Ok(UnaryOps::Cosine(kw::cos(span))),
                    "exp" => return Ok(UnaryOps::Exp(kw::exp(span))),
                    "ln" => return Ok(UnaryOps::Ln(kw::ln(span))),
                    "sin" => return Ok(UnaryOps::Sine(kw::sin(span))),
                    "tan" => return Ok(UnaryOps::Tan(kw::tan(span))),
                    _ => return Err(input.error("Method not found")),
                }
            }
            if input.peek2(kw::cos) {
                input.parse::<kw::cos>().map(UnaryOps::Cosine)
            } else if input.peek2(kw::sin) {
                input.parse::<kw::sin>().map(UnaryOps::Sine)
            } else if input.peek2(kw::tan) {
                input.parse::<kw::tan>().map(UnaryOps::Tan)
            } else if input.peek2(kw::ln) {
                input.parse::<kw::ln>().map(UnaryOps::Ln)
            } else if input.peek2(kw::exp) {
                input.parse::<kw::exp>().map(UnaryOps::Exp)
            } else {
                Err(input.error("Expected a method call"))
            }
        } else {
            input.parse::<syn::UnOp>().map(UnaryOps::Std)
        }
    }
}
