/*
    Appellation: error <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
use super::kinds::{ErrorKind, ExternalError, SyncError};
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
    /// Get an owned reference to the error kind
    pub fn kind(&self) -> &ErrorKind {
        &self.kind
    }
    /// Get an owned reference to the error message
    pub fn message(&self) -> &str {
        &self.message
    }
    /// Set the error message
    pub fn set_message(&mut self, msg: impl ToString) {
        self.message = msg.to_string();
    }
    /// Consume the error and return the message
    pub fn into_message(self) -> String {
        self.message
    }
    /// A functional method for setting the error kind
    pub fn with_kind(mut self, kind: ErrorKind) -> Self {
        self.kind = kind;
        self
    }
    /// A functional method for setting the error message
    pub fn with_message(mut self, msg: impl ToString) -> Self {
        self.message = msg.to_string();
        self
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
        Self::new(ErrorKind::Sync(SyncError::TryLock), err.to_string())
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

macro_rules! err_variant {
    (external $variant:ident, $t:ty) => {
        impl From<$t> for Error {
            fn from(err: $t) -> Self {
                Self::new(
                    ErrorKind::External(ExternalError::$variant),
                    err.to_string(),
                )
            }
        }
    };
}

err_variant!(external Unknown, &str);
err_variant!(external Unknown, String);
err_variant!(external Unknown, Box<dyn std::error::Error>);
