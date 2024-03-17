/*
    Appellation: error <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
use super::kinds::ErrorKind;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[cfg_attr(feature = "serde", derive(Deserialize, Serialize,))]
#[derive(Clone, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Error {
    kind: ErrorKind,
    message: String,
}

impl Error {
    pub fn new(kind: ErrorKind, msg: impl ToString) -> Self {
        Self {
            kind,
            message: msg.to_string(),
        }
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}: {}", self.kind, self.message)
    }
}

impl std::error::Error for Error {}

impl From<ErrorKind> for Error {
    fn from(kind: ErrorKind) -> Self {
        Self::new(kind, "")
    }
}

impl<T> From<std::sync::TryLockError<T>> for Error {
    fn from(err: std::sync::TryLockError<T>) -> Self {
        Self::new(ErrorKind::Sync, err.to_string())
    }
}

impl<E> From<petgraph::algo::Cycle<E>> for Error
where
    E: Copy + std::fmt::Debug,
{
    fn from(err: petgraph::algo::Cycle<E>) -> Self {
        Self::new(ErrorKind::Graph, format!("Cycle: {:?}", err.node_id()))
    }
}

impl From<petgraph::algo::NegativeCycle> for Error {
    fn from(_err: petgraph::algo::NegativeCycle) -> Self {
        Self::new(ErrorKind::Graph, "Negative Cycle detected")
    }
}

macro_rules! error_from {
    (shared $kind:expr, ($($t:ty),*)) => {
        $(
            error_from!($kind, $t);
        )*
    };
    ($kind:expr, $t:ty) => {
        impl From<$t> for Error {
            fn from(err: $t) -> Self {
                Self::new($kind, err.to_string())
            }
        }
    };
}

error_from!(shared ErrorKind::Unknown, (&str, String, Box<dyn std::error::Error>));
