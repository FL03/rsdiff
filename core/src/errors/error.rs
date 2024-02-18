/*
    Appellation: error <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
use super::kinds::ErrorKind;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
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

impl From<Box<dyn std::error::Error>> for Error {
    fn from(err: Box<dyn std::error::Error>) -> Self {
        Self::new(ErrorKind::Unknown, err.to_string())
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
    fn from(err: petgraph::algo::NegativeCycle) -> Self {
        Self::new(ErrorKind::Graph, "Negative Cycle detected")
    }
}
