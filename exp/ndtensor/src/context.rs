/*
    Appellation: context <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
use crate::TensorKind;

#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub struct Context {
    pub(crate) kind: TensorKind,
}

impl Context {
    pub fn new(kind: bool) -> Self {
        Self { kind: kind.into() }
    }

    pub fn into_var(self) -> Self {
        Self {
            kind: TensorKind::Variable,
        }
    }

    pub fn kind(&self) -> TensorKind {
        self.kind
    }

    pub fn is_variable(&self) -> bool {
        self.kind().is_variable()
    }

    pub fn set_kind(&mut self, kind: TensorKind) {
        self.kind = kind;
    }
}
