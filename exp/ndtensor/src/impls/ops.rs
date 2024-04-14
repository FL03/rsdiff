/*
    Appellation: ops <mod>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
use crate::prelude::{Dimension, TensorExpr};
use crate::tensor::{new, TensorBase};
use acme::prelude::{BinaryOp, Scalar, UnaryOp};
use ndarray::DimMax;
use ndarray::{Data, DataMut, DataOwned, RawDataClone};

macro_rules! unop {
    ($(($method:ident, $op:ident)),*) => {
        $(
            unop!($method, $op);
        )*
    };
    ($method:ident, $op:ident) => {
        pub fn $method(&self) -> crate::Tensor<A, D> {
            let data = self.data().mapv(|x| x.$method());
            let op = TensorExpr::Unary {
                recv: Box::new(self.clone().into_dyn().into_owned()),
                op: UnaryOp::$op,
            };
            new(data, Some(op))
        }
    };
}

#[allow(unused_macros)]
macro_rules! binop {
    ($(($method:ident, $op:ident)),*) => {
        $(
            stdop!($method, $op);
        )*
    };
    ($method:ident, $op:ident) => {
        pub fn $method(&self, other: &Self) -> Self {
            let data = self.data() + other.data();
            let op = TensorExpr::binary(
                Box::new(self.clone().into_dyn()),
                Box::new(other.clone().into_dyn()),
                BinaryOp::$op,
            );
            new(data, Some(op))
        }
    };

}

impl<A, S, D> TensorBase<S, D>
where
    A: Scalar,
    D: Dimension,
    S: Data<Elem = A> + DataOwned + RawDataClone,
{
    pub fn abs(&self) -> crate::Tensor<<A as Scalar>::Real, D>
    where
        A: Scalar<Real = A>,
    {
        let data = self.data().mapv(|x| x.abs());
        let op =
            TensorExpr::<S, S>::unary(Box::new(self.clone().into_dyn()), UnaryOp::Abs).into_owned();
        TensorBase::from_arr(data).with_op(op)
    }
    unop!(
        (cos, Cos),
        (cosh, Cosh),
        (exp, Exp),
        (ln, Ln),
        (sin, Sin),
        (sinh, Sinh),
        (sqr, Square),
        (sqrt, Sqrt),
        (tan, Tan),
        (tanh, Tanh)
    );
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
            type Output = TensorBase<S1, <D1 as DimMax<D2>>::Output>;

            #[allow(unused_variables)]
            fn $call(self, rhs: TensorBase<S2, D2>) -> Self::Output {
                let data = core::ops::$bound::$call(self.data(), rhs.data());
                let op = TensorExpr::binary(
                    Box::new(self.into_dyn().into_owned()),
                    Box::new(rhs.into_dyn().into_owned()),
                    BinaryOp::$call(),
                );
                // new(data, Some(op))
                unimplemented!()
            }
        }
    };

}

stdop!(
    (Add, add, +)

);
