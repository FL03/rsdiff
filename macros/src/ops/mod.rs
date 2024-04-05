/*
    Appellation: ops <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
//! # Operations
//!
pub use self::unary::*;

pub(crate) mod unary;

use std::str::FromStr;

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum Methods {
    Unary(UnaryMethod),
}

impl FromStr for Methods {
    type Err = Box<dyn std::error::Error>;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        if let Ok(method) = UnaryMethod::from_str(s) {
            return Ok(Methods::Unary(method));
        }

        Err("Method not found".into())
    }
}
