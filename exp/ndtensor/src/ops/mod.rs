/*
    Appellation: ops <mod>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
pub use self::expr::*;

pub(crate) mod expr;

use ndarray::*;

pub type TOp<A, B> = TensorOp<OwnedArcRepr<A>, OwnedArcRepr<B>>;

pub struct TensorOp<S1, S2 = S1>(pub(crate) Option<TensorExpr<S1, S2>>)
where
    S1: RawData,
    S2: RawData;

impl<A, B, S1, S2> TensorOp<S1, S2>
where
    S1: RawData<Elem = A>,
    S2: RawData<Elem = B>,
{
    pub fn new(expr: Option<TensorExpr<S1, S2>>) -> Self {
        TensorOp(expr)
    }

    pub fn none() -> Self {
        TensorOp(None)
    }

    pub fn as_ref(&self) -> Option<&TensorExpr<S1, S2>> {
        self.0.as_ref()
    }

    pub fn as_mut(&mut self) -> Option<&mut TensorExpr<S1, S2>> {
        self.0.as_mut()
    }

    pub fn into_owned(self) -> TensorOp<OwnedRepr<A>, OwnedRepr<B>>
    where
        A: Clone,
        B: Clone,
        S1: DataOwned,
        S2: DataOwned,
    {
        TensorOp(self.0.map(|expr| expr.into_owned()))
    }

    pub fn into_shared(self) -> TensorOp<OwnedArcRepr<A>, OwnedArcRepr<B>>
    where
        S1: DataOwned,
        S2: DataOwned,
    {
        TensorOp(self.0.map(|expr| expr.into_shared()))
    }

    pub fn is_none(&self) -> bool {
        self.0.is_none()
    }

    pub fn is_some(&self) -> bool {
        self.0.is_some()
    }

    pub fn view(&self) -> TensorOp<ViewRepr<&'_ A>, ViewRepr<&'_ B>>
    where
        S1: Data,
        S2: Data,
    {
        TensorOp(self.0.as_ref().map(|expr| expr.view()))
    }
}

impl<S1, S2> Clone for TensorOp<S1, S2>
where
    S1: RawDataClone,
    S2: RawDataClone,
{
    fn clone(&self) -> Self {
        TensorOp(self.0.clone())
    }
}

impl<S1, S2> Default for TensorOp<S1, S2>
where
    S1: RawData,
    S2: RawData,
{
    fn default() -> Self {
        TensorOp(None)
    }
}

impl<S1, S2> From<TensorOp<S1, S2>> for Option<TensorExpr<S1, S2>>
where
    S1: RawData,
    S2: RawData,
{
    fn from(op: TensorOp<S1, S2>) -> Self {
        op.0
    }
}

impl<S1, S2> From<Option<TensorExpr<S1, S2>>> for TensorOp<S1, S2>
where
    S1: RawData,
    S2: RawData,
{
    fn from(expr: Option<TensorExpr<S1, S2>>) -> Self {
        TensorOp(expr)
    }
}

impl<S1, S2> From<TensorExpr<S1, S2>> for TensorOp<S1, S2>
where
    S1: RawData,
    S2: RawData,
{
    fn from(expr: TensorExpr<S1, S2>) -> Self {
        TensorOp(Some(expr))
    }
}
