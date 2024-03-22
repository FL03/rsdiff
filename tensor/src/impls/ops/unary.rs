/*
    Appellation: arith <mod>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
use crate::prelude::{Scalar, TensorOp};
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
        let op = TensorOp::Unary(Box::new(self), UnaryOp::Neg);
        from_vec_with_op(op, shape, store)
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
        let op = TensorOp::Unary(Box::new(self.clone()), UnaryOp::Neg);
        from_vec_with_op(op, shape, store)
    }
}

macro_rules! impl_unary_arith {
    ($variant:ident, $method:ident, $e:expr) => {
        impl<T> TensorBase<T>
        where
            T: Scalar,
        {
            pub fn $method(self) -> Self {
                let shape = self.shape().clone();
                let store = self.store.iter().map($e).collect();
                let op = TensorOp::<T>::Unary(Box::new(self), UnaryOp::$variant);
                from_vec_with_op(op, shape, store)
            }
        }
    };
}

impl_unary_arith!(Exp, exp, |v| v.exp());
// impl_unary_arith!(Log, log, |v| v.log());

impl_unary_arith!(Cos, cos, |v| v.cos());
impl_unary_arith!(Cosh, cosh, |v| v.cosh());
impl_unary_arith!(Sin, sin, |v| v.sin());
impl_unary_arith!(Sinh, sinh, |v| v.sinh());
impl_unary_arith!(Sqrt, sqrt, |v| v.sqrt());
impl_unary_arith!(Tan, tan, |v| v.tan());
impl_unary_arith!(Tanh, tanh, |v| v.tanh());
