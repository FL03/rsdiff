/*
    Appellation: ops <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
//! # Operations
//!
pub use self::{binary::*, unary::*};

pub(crate) mod binary;
pub(crate) mod unary;

use core::str::FromStr;
use proc_macro2::TokenStream;
use syn::{ExprMethodCall, Ident};

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[non_exhaustive]
pub enum Methods {
    Binary(BinaryOp),
    Unary(UnaryOp),
}

impl Methods {
    pub fn binary(op: BinaryOp) -> Self {
        Methods::Binary(op)
    }

    pub fn unary(op: UnaryOp) -> Self {
        Methods::Unary(op)
    }

    pub fn from_bin_op(op: syn::BinOp) -> Self {
        Self::binary(BinaryOp::from_binary(op).expect("Unsupported binary operation"))
    }

    pub fn from_method_call(expr: &ExprMethodCall, var: &Ident) -> TokenStream {
        let ExprMethodCall {
            args,
            method,
            receiver,
            ..
        } = expr;
        let method_name = method.clone().to_string();

        if let Ok(method) = Methods::from_str(&method_name) {
            return match method {
                Methods::Binary(method) => method.grad(&receiver, &args[0], var),
                Methods::Unary(method) => method.grad(&receiver, var),
                // _ => panic!("Unsupported method"),
            };
        }
        panic!("Unsupported method");
    }
}

impl FromStr for Methods {
    type Err = Box<dyn std::error::Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Ok(method) = BinaryOp::from_str(s) {
            return Ok(Methods::Binary(method));
        }
        if let Ok(method) = UnaryOp::from_str(s) {
            return Ok(Methods::Unary(method));
        }

        Err("Method not found".into())
    }
}

impl From<BinaryOp> for Methods {
    fn from(op: BinaryOp) -> Self {
        Methods::binary(op)
    }
}

impl From<UnaryOp> for Methods {
    fn from(op: UnaryOp) -> Self {
        Methods::unary(op)
    }
}
