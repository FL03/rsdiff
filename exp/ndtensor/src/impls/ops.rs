/*
    Appellation: ops <mod>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
use crate::prelude::{Dimension, TensorExpr};
use crate::TensorBase;
use acme::prelude::{BinaryOp, UnaryOp};
use ndarray::{Data, DataMut, DataOwned, OwnedRepr, RawDataClone};
use ndarray::{DimMax, Ix0};
use num::complex::ComplexFloat;

macro_rules! unop {
    ($($method:ident),*) => {
        $(
            unop!(@loop $method);
        )*
    };
    (@loop $method:ident) => {
        pub fn $method(&self) -> crate::Tensor<A, D> {
            let data = self.data().mapv(|x| x.$method());
            let op = TensorExpr::Unary {
                recv: Box::new(self.clone().into_dyn().into_owned()),
                op: UnaryOp::$method(),
            };
            new!(data, Some(op))
        }
    };
}

macro_rules! binop {
    ($(($method:ident, $op:tt)),*) => {
        $(
            binop!(@loop $method, $op);
        )*
    };
    (@loop $method:ident, $op:tt) => {
        pub fn $method(&self, other: &Self) -> crate::Tensor<A, D> {
            let data = self.data() $op other.data();
            let op = TensorExpr::binary(
                self.clone().into_dyn().boxed(),
                other.clone().into_dyn().boxed(),
                BinaryOp::$method(),
            );
            new!(data, Some(op.into_owned()))
        }
    };

}

impl<A, S, D> TensorBase<S, D>
where
    A: ComplexFloat,
    D: Dimension,
    S: Data<Elem = A> + DataOwned + RawDataClone,
{
    pub fn abs(&self) -> crate::Tensor<<A as ComplexFloat>::Real, D>
    where
        A: ComplexFloat<Real = A>,
    {
        let data = self.data().mapv(|x| x.abs());
        let op = TensorExpr::<S, S>::unary(self.clone().into_dyn().boxed(), UnaryOp::Abs);
        TensorBase::from_arr(data).with_op(op.into_owned())
    }
    binop!(
        (add, +),
        (div, /),
        (mul, *),
        (rem, %),
        (sub, -)
    );
    unop!(acos, acosh, asin, asinh, atan, cos, cosh, exp, ln, neg, sin, sinh, sqrt, tan, tanh);
}

macro_rules! stdop {
    ($(($bound:ident, $call:ident, $op:tt)),*) => {
        $(
            stdop!($bound, $call, $op);
        )*
    };
    ($bound:ident, $call:ident, $op:tt) => {


        impl<A, B, S1, S2, D1, D2> core::ops::$bound<TensorBase<S2, D2>> for TensorBase<S1, D1>
        where
            A: Clone + core::ops::$bound<B, Output = A>,
            B: Clone,
            D1: Dimension + DimMax<D2>,
            D2: Dimension,
            S1: DataOwned<Elem = A> + DataMut,
            S2: DataOwned<Elem = B>,

        {
            type Output = TensorBase<OwnedRepr<A>, <D1 as DimMax<D2>>::Output>;

            fn $call(self, rhs: TensorBase<S2, D2>) -> Self::Output {
                let data = core::ops::$bound::$call(self.data(), rhs.data());
                let lhs = self.into_dyn().into_owned();
                let op = unsafe { TensorExpr::binary(
                    Box::new(lhs),
                    Box::new(rhs.into_dyn().raw_view().cast::<A>().deref_into_view()),
                    BinaryOp::$call(),
                )};
                new!(data, Some(op.to_owned()))
            }
        }

        impl<'a, A, B, S1, S2, D1, D2> core::ops::$bound<TensorBase<S2, D2>> for &'a TensorBase<S1, D1>
        where
            A: Clone + core::ops::$bound<B, Output = A>,
            B: Clone,
            D1: Dimension + DimMax<D2>,
            D2: Dimension,
            S1: DataOwned<Elem = A> + DataMut + RawDataClone,
            S2: DataOwned<Elem = B>,

        {
            type Output = TensorBase<OwnedRepr<A>, <D1 as DimMax<D2>>::Output>;

            fn $call(self, rhs: TensorBase<S2, D2>) -> Self::Output {
                let data = core::ops::$bound::$call(self.data(), rhs.data());
                let lhs = self.clone().into_dyn().into_owned();
                let op = unsafe { TensorExpr::binary(
                    Box::new(lhs),
                    Box::new(rhs.into_dyn().raw_view().cast::<A>().deref_into_view()),
                    BinaryOp::$call(),
                )};
                new!(data, Some(op.to_owned()))
            }
        }
    };

}

stdop!(
    (Add, add, +),
    (Div, div, /),
    (Mul, mul, *),
    (Rem, rem, %),
    (Sub, sub, -)

);
