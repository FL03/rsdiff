/*
    Appellation: arith <mod>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
use crate::prelude::{Scalar, ScalarExt, TensorExpr};
use crate::tensor::*;
use acme::ops::unary::UnaryOp;
use core::ops;

impl<T> ops::Neg for TensorBase<T>
where
    T: Copy + ops::Neg<Output = T>,
{
    type Output = Self;

    fn neg(self) -> Self::Output {
        let shape = self.shape().clone();
        let store = self.data().iter().map(|a| (*a).neg()).collect();
        let op = TensorExpr::unary(self, UnaryOp::Neg);
        from_vec_with_op(false, op, shape, store)
    }
}

impl<'a, T> ops::Neg for &'a TensorBase<T>
where
    T: Copy + ops::Neg<Output = T>,
{
    type Output = TensorBase<T>;

    fn neg(self) -> Self::Output {
        let shape = self.shape();
        let store = self.data().iter().map(|a| (*a).neg()).collect();
        let op = TensorExpr::unary(self.clone(), UnaryOp::Neg);
        from_vec_with_op(false, op, shape, store)
    }
}

impl<T> ops::Not for TensorBase<T>
where
    T: Copy + ops::Not<Output = T>,
{
    type Output = Self;

    fn not(self) -> Self::Output {
        let shape = self.shape().clone();
        let store = self.data().iter().map(|a| (*a).not()).collect();
        let op = TensorExpr::unary(self, UnaryOp::Not);
        from_vec_with_op(false, op, shape, store)
    }
}

impl<'a, T> ops::Not for &'a TensorBase<T>
where
    T: Copy + ops::Not<Output = T>,
{
    type Output = TensorBase<T>;

    fn not(self) -> Self::Output {
        let shape = self.shape();
        let store = self.data.iter().copied().map(|a| !a).collect();
        let op = TensorExpr::unary(self.clone(), UnaryOp::Not);
        from_vec_with_op(false, op, shape, store)
    }
}

macro_rules! impl_unary_op {
    ($variant:ident, $method:ident) => {
        pub fn $method(&self) -> Self {
            let shape = self.shape();
            let store = self.data().iter().copied().map(|v| v.$method()).collect();
            let op = TensorExpr::unary(self.clone(), UnaryOp::$variant);
            from_vec_with_op(false, op, shape, store)
        }
    };
    (custom $variant:ident, $method:ident, $f:expr) => {
        pub fn $method(self) -> Self {
            let shape = self.shape().clone();
            let store = self.store.iter().copied().map($f).collect();
            let op = TensorExpr::unary(self, UnaryOp::$variant);
            from_vec_with_op(false, op, shape, store)
        }
    };
}

impl<T> TensorBase<T>
where
    T: Scalar,
{
    pub fn abs(&self) -> TensorBase<<T as Scalar>::Real>
    where
        T: Scalar<Real = T>,
    {
        let shape = self.shape();
        let store = self.data.iter().copied().map(Scalar::abs).collect();
        let op = TensorExpr::unary(self.clone(), UnaryOp::Abs);
        from_vec_with_op(false, op, shape, store)
    }

    pub fn sigmoid(&self) -> TensorBase<T>
    where
        T: ScalarExt,
    {
        let shape = self.shape();
        let store = self.data().iter().copied().map(ScalarExt::sigmoid).collect();
        let op = TensorExpr::sigmoid(self.clone());
        from_vec_with_op(false, op, shape, store)
    }

    impl_unary_op!(Cos, cos);
    impl_unary_op!(Cosh, cosh);
    impl_unary_op!(Exp, exp);
    impl_unary_op!(Ln, ln);
    impl_unary_op!(Recip, recip);
    impl_unary_op!(Sin, sin);
    impl_unary_op!(Sinh, sinh);
    impl_unary_op!(Square, sqr);
    impl_unary_op!(Sqrt, sqrt);
    impl_unary_op!(Tan, tan);
    impl_unary_op!(Tanh, tanh);
}
