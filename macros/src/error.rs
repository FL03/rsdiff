/*
    Appellation: error <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/

#[derive(Clone, Debug)]
pub enum Error {
    Ast(String),
    Syn(syn::Error),
    Unknown(String),
}

impl core::fmt::Display for Error {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        let err = match self {
            Error::Ast(err) => err.to_string(),
            Error::Syn(err) => err.to_string(),
            Error::Unknown(msg) => msg.to_string(),
        };
        write!(f, "{err}")
    }
}

impl std::error::Error for Error {}

impl From<&str> for Error {
    fn from(msg: &str) -> Self {
        Error::Unknown(msg.to_string())
    }
}

macro_rules! from_err {
    ($err:ident($($from:ident)::*)) => {
        from_err!(@impl $err($($from)::*));
    };
    (@impl $err:ident($($from:ident)::*)) => {
        impl From<$($from)::*> for Error {
            fn from(err: $($from)::*) -> Self {
                Error::$err(err)
            }
        }
    };
}

from_err!(Syn(syn::Error));

from_err!(Unknown(String));
