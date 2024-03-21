/*
    Appellation: arith <mod>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
use crate::ops::{BinaryOp, Op, UnaryOp};
use crate::prelude::Scalar;
use crate::tensor::*;

impl<T> std::ops::Neg for TensorBase<T>
where
    T: Copy + std::ops::Neg<Output = T>,
{
    type Output = Self;

    fn neg(self) -> Self::Output {
        let shape = self.shape().clone();
        let store = self.data().iter().map(|a| -*a).collect();
        let op = Op::Unary(Box::new(self), UnaryOp::Neg);
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
        let store = self.data().iter().map(|a| -*a).collect();
        let op = Op::Unary(Box::new(self.clone()), UnaryOp::Neg);
        from_vec_with_op(op, shape, store)
    }
}

macro_rules! cmp {
    (ne: $lhs:expr, $rhs:expr) => {
        if $lhs != $rhs {
            panic!("Shape Mismatch: {:?} != {:?}", $lhs, $rhs);
        }
    };
}

macro_rules! impl_arith {
    ($trait:ident, $method:ident, $op:tt) => {
        impl<T> std::ops::$trait for TensorBase<T>
        where
            T: Scalar + std::ops::$trait<Output = T>,
        {
            type Output = Self;

            fn $method(self, other: Self) -> Self::Output {
                cmp!(ne: self.shape(), other.shape());
                let shape = self.shape().clone();
                let store = self.data().iter().zip(other.data().iter()).map(|(a, b)| *a $op *b).collect();
                let op = Op::Binary(Box::new(self), Box::new(other), BinaryOp::$trait);
                from_vec_with_op(op, shape, store)
            }
        }

        impl<'a, T> std::ops::$trait<&'a TensorBase<T>> for TensorBase<T>
        where
            T: Scalar + std::ops::$trait<Output = T>,
        {
            type Output = TensorBase<T>;

            fn $method(self, other: &'a TensorBase<T>) -> Self::Output {
                if self.shape() != other.shape() {
                    panic!("shapes must be equal");
                }
                let shape = self.shape().clone();
                let store = self.data().iter().zip(other.data().iter()).map(|(a, b)| *a $op *b).collect();
                let op = Op::Binary(Box::new(self), Box::new(other.clone()), BinaryOp::$trait);
                from_vec_with_op(op, shape, store)
            }
        }

        impl<'a, T> std::ops::$trait<TensorBase<T>> for &'a TensorBase<T>
        where
            T: Scalar + std::ops::$trait<Output = T>,
        {
            type Output = TensorBase<T>;

            fn $method(self, other: TensorBase<T>) -> Self::Output {
                if self.shape() != other.shape() {
                    panic!("shapes must be equal");
                }
                let shape = self.shape().clone();
                let store = self.data().iter().zip(other.data().iter()).map(|(a, b)| *a $op *b).collect();
                let op = Op::Binary(Box::new(self.clone()), Box::new(other), BinaryOp::$trait);
                from_vec_with_op(op, shape, store)
            }
        }

        impl<'a, 'b, T> std::ops::$trait<&'b TensorBase<T>> for &'a TensorBase<T>
        where
            T: Scalar + std::ops::$trait<Output = T>,
        {
            type Output = TensorBase<T>;

            fn $method(self, other: &'b TensorBase<T>) -> Self::Output {
                if self.shape() != other.shape() {
                    panic!("shapes must be equal");
                }
                let shape = self.shape().clone();
                let store = self.data().iter().zip(other.data().iter()).map(|(a, b)| *a $op *b).collect();
                let op = Op::Binary(Box::new(self.clone()), Box::new(other.clone()), BinaryOp::$trait);
                from_vec_with_op(op, shape, store)
            }
        }
    };
}

macro_rules! impl_scalar_arith {
    ($trait:ident, $method:ident, $op:tt) => {
        // impl<T> TensorBase<T>
        // where
        //     T: Copy + std::ops::$trait<Output = T>,
        // {
        //     pub fn $method(self, other: T) -> TensorBase<T> {
        //         let store = self.store.iter().map(|a| *a $op other).collect();
        //         from_vec(self.shape().clone(), store)
        //     }
        // }

        impl<T> std::ops::$trait<T> for TensorBase<T>
        where
            T: Copy + std::ops::$trait<Output = T>,
        {
            type Output = Self;

            fn $method(self, other: T) -> Self::Output {
                let shape = self.shape().clone();
                let store = self.into_store().iter().map(|a| *a $op other).collect();
                Self::Output::from_vec(shape, store)
            }
        }

        impl<'a, T> std::ops::$trait<T> for &'a TensorBase<T>
        where
            T: Copy + std::ops::$trait<Output = T>,
        {
            type Output = TensorBase<T>;

            fn $method(self, other: T) -> Self::Output {
                let shape = self.shape().clone();
                let store = self.data().iter().map(|a| *a $op other).collect();
                Self::Output::from_vec(shape, store)
            }
        }
    };
}

macro_rules! impl_assign_op {
    ($trait:ident, $method:ident, $inner:ident, $op:tt) => {
        impl<T> std::ops::$trait for TensorBase<T>
        where
            T: Copy + std::ops::$inner<T, Output = T>,
        {
            fn $method(&mut self, other: Self) {
                cmp!(ne: self.shape(), other.shape());
                self.store = self.store.iter().zip(other.store.iter()).map(|(a, b)| *a $op *b).collect();
            }
        }

        impl<'a, T> std::ops::$trait<&'a TensorBase<T>> for TensorBase<T>
        where
            T: Copy + std::ops::$inner<Output = T>,
        {
            fn $method(&mut self, other: &'a TensorBase<T>) {
                cmp!(ne: self.shape(), other.shape());
                self.store = self.store.iter().zip(other.store.iter()).map(|(a, b)| *a $op *b).collect();
            }
        }
    };

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
                let op = Op::<T>::Unary(Box::new(self), UnaryOp::$variant);
                from_vec_with_op(op, shape, store)
            }
        }
    };
}

impl_arith!(Add, add, +);
impl_arith!(Div, div, /);
impl_arith!(Mul, mul, *);
impl_arith!(Sub, sub, -);

impl_assign_op!(AddAssign, add_assign, Add, +);
impl_assign_op!(DivAssign, div_assign, Div, /);
impl_assign_op!(MulAssign, mul_assign, Mul, *);
impl_assign_op!(SubAssign, sub_assign, Sub, -);

impl_scalar_arith!(Add, add, +);
impl_scalar_arith!(Div, div, /);
impl_scalar_arith!(Mul, mul, *);
impl_scalar_arith!(Sub, sub, -);

impl_unary_arith!(Exp, exp, |v| v.exp());
// impl_unary_arith!(Log, log, |v| v.log());

impl_unary_arith!(Cos, cos, |v| v.cos());
impl_unary_arith!(Cosh, cosh, |v| v.cosh());
impl_unary_arith!(Sin, sin, |v| v.sin());
impl_unary_arith!(Sinh, sinh, |v| v.sinh());
impl_unary_arith!(Sqrt, sqrt, |v| v.sqrt());
impl_unary_arith!(Tan, tan, |v| v.tan());
impl_unary_arith!(Tanh, tanh, |v| v.tanh());
