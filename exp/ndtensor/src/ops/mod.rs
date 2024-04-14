use ndarray::{DataOwned, OwnedArcRepr, OwnedRepr, RawData, RawDataClone};

/*
    Appellation: ops <mod>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
pub use self::expr::*;

pub(crate) mod expr;

pub struct TensorOp<S>(pub(crate) Option<TensorExpr<S>>)
where
    S: RawData;

impl<A, S> TensorOp<S>
where
    S: RawData<Elem = A>,
{
    pub fn new(expr: Option<TensorExpr<S>>) -> Self {
        TensorOp(expr)
    }

    pub fn none() -> Self {
        TensorOp(None)
    }

    pub fn as_ref(&self) -> Option<&TensorExpr<S>> {
        self.0.as_ref()
    }

    pub fn as_mut(&mut self) -> Option<&mut TensorExpr<S>> {
        self.0.as_mut()
    }

    pub fn into_owned(self) -> TensorOp<OwnedRepr<A>>
    where
        A: Clone,
        S: DataOwned,
    {
        TensorOp(self.0.map(|expr| expr.into_owned()))
    }

    pub fn into_shared(self) -> TensorOp<OwnedArcRepr<A>>
    where
        S: DataOwned,
    {
        TensorOp(self.0.map(|expr| expr.into_shared()))
    }

    pub fn is_none(&self) -> bool {
        self.0.is_none()
    }

    pub fn is_some(&self) -> bool {
        self.0.is_some()
    }
}

impl<S> Clone for TensorOp<S>
where
    S: RawDataClone,
{
    fn clone(&self) -> Self {
        TensorOp(self.0.clone())
    }
}

impl<S> From<TensorOp<S>> for Option<TensorExpr<S>>
where
    S: RawData,
{
    fn from(op: TensorOp<S>) -> Self {
        op.0
    }
}

impl<S> From<Option<TensorExpr<S>>> for TensorOp<S>
where
    S: RawData,
{
    fn from(expr: Option<TensorExpr<S>>) -> Self {
        TensorOp(expr)
    }
}

impl<S> From<TensorExpr<S>> for TensorOp<S>
where
    S: RawData,
{
    fn from(expr: TensorExpr<S>) -> Self {
        TensorOp(Some(expr))
    }
}
