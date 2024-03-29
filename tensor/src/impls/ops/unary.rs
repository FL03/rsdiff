/*
    Appellation: arith <mod>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
use crate::prelude::{Scalar, TensorExpr};
use crate::tensor::*;
use acme::ops::unary::UnaryOp;

impl<T> std::ops::Neg for TensorBase<T>
where
    T: Copy + std::ops::Neg<Output = T>,
{
    type Output = Self;

    fn neg(self) -> Self::Output {
        let shape = self.shape().clone();
        let store = self.data().iter().copied().map(|a| -a).collect();
        let op = TensorExpr::unary(self, UnaryOp::Neg);
        from_vec_with_op(false, op, shape, store)
    }
}

impl<'a, T> std::ops::Neg for &'a TensorBase<T>
where
    T: Copy + std::ops::Neg<Output = T>,
{
    type Output = TensorBase<T>;

    fn neg(self) -> Self::Output {
        let shape = self.shape().clone();
        let store = self.data().iter().copied().map(|a| -a).collect();
        let op = TensorExpr::unary(self.clone(), UnaryOp::Neg);
        from_vec_with_op(false, op, shape, store)
    }
}

impl<T> std::ops::Not for TensorBase<T>
where
    T: Copy + std::ops::Not<Output = T>,
{
    type Output = Self;

    fn not(self) -> Self::Output {
        let shape = self.shape().clone();
        let store = self.data().iter().copied().map(|a| !a).collect();
        let op = TensorExpr::unary(self, UnaryOp::Not);
        from_vec_with_op(false, op, shape, store)
    }
}

impl<'a, T> std::ops::Not for &'a TensorBase<T>
where
    T: Copy + std::ops::Not<Output = T>,
{
    type Output = TensorBase<T>;

    fn not(self) -> Self::Output {
        let shape = self.shape().clone();
        let store = self.data().iter().copied().map(|a| !a).collect();
        let op = TensorExpr::unary(self.clone(), UnaryOp::Not);
        from_vec_with_op(false, op, shape, store)
    }
}

macro_rules! impl_unary_op {
    ($variant:ident, $method:ident) => {
        pub fn $method(self) -> Self {
            let shape = self.shape().clone();
            let store = self.store.iter().copied().map(|v| v.$method()).collect();
            let op = TensorExpr::unary(self, UnaryOp::$variant);
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
    pub fn abs(self) -> TensorBase<<T as Scalar>::Real>
    where
        T: Scalar<Real = T>,
    {
        let shape = self.shape().clone();
        let store = self.store.iter().copied().map(|v| v.abs()).collect();
        let op = TensorExpr::unary(self, UnaryOp::Abs);
        from_vec_with_op(false, op, shape, store)
    }

    impl_unary_op!(Cos, cos);
    impl_unary_op!(Cosh, cosh);
    impl_unary_op!(Exp, exp);
    impl_unary_op!(Ln, ln);
    impl_unary_op!(Sin, sin);
    impl_unary_op!(Sinh, sinh);
    impl_unary_op!(Square, sqr);
    impl_unary_op!(Sqrt, sqrt);
    impl_unary_op!(Tan, tan);
    impl_unary_op!(Tanh, tanh);
}
