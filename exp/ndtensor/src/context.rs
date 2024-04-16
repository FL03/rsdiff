/*
    Appellation: context <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
use crate::TensorKind;

use ndarray::{ArrayBase, Dimension, IntoDimension, RawData};

#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub struct Context {
    pub(crate) kind: TensorKind,
    pub(crate) rank: usize,
}

impl Context {
    pub fn new(kind: bool, rank: usize) -> Self {
        Self {
            kind: TensorKind::new(kind),
            rank,
        }
    }

    pub fn variable(mut self) -> Self {
        self.kind = TensorKind::Variable;
        self
    }

    pub fn from_shape<D>(shape: impl IntoDimension<Dim = D>) -> Self
    where
        D: Dimension,
    {
        Self::new(false, shape.into_dimension().ndim())
    }

    pub fn from_arr<S, D>(arr: ArrayBase<S, D>) -> Self
    where
        D: Dimension,
        S: RawData,
    {
        Self::new(false, arr.ndim())
    }

    pub fn into_var(self) -> Self {
        Self {
            kind: TensorKind::Variable,
            ..self
        }
    }

    pub fn kind(&self) -> TensorKind {
        self.kind
    }

    pub fn is_variable(&self) -> bool {
        self.kind().is_variable()
    }

    pub fn rank(&self) -> usize {
        self.rank
    }

    pub fn set_kind(&mut self, kind: TensorKind) {
        self.kind = kind;
    }
}
