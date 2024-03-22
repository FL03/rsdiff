/*
    Appellation: arith <mod>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
use crate::prelude::{Scalar, TensorOp};
use crate::tensor::*;
use acme::ops::binary::BinaryOp;

macro_rules! cmp {
    (ne: $lhs:expr, $rhs:expr) => {
        if $lhs != $rhs {
            panic!("Shape Mismatch: {:?} != {:?}", $lhs, $rhs);
        }
    };
}

macro_rules! impl_arithmetic {
    ($trait:ident, $method:ident, $op:tt) => {
        impl_scalar_arith!($trait, $method, $op);

        impl<T> std::ops::$trait for TensorBase<T>
        where
            T: Scalar + std::ops::$trait<Output = T>,
        {
            type Output = Self;

            fn $method(self, other: Self) -> Self::Output {
                cmp!(ne: self.shape(), other.shape());
                let shape = self.shape().clone();
                let store = self.data().iter().zip(other.data().iter()).map(|(a, b)| *a $op *b).collect();
                let op = TensorOp::Binary(Box::new(self), Box::new(other), BinaryOp::$trait);
                from_vec_with_op(false, op, shape, store)
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
                let op = TensorOp::Binary(Box::new(self), Box::new(other.clone()), BinaryOp::$trait);
                from_vec_with_op(false, op, shape, store)
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
                let op = TensorOp::Binary(Box::new(self.clone()), Box::new(other), BinaryOp::$trait);
                from_vec_with_op(false, op, shape, store)
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
                let op = TensorOp::Binary(Box::new(self.clone()), Box::new(other.clone()), BinaryOp::$trait);
                from_vec_with_op(false, op, shape, store)
            }
        }
    };
}

macro_rules! impl_scalar_arith {
    ($trait:ident, $method:ident, $op:tt) => {

        impl<T> std::ops::$trait<T> for TensorBase<T>
        where
            T: Copy + std::ops::$trait<Output = T>,
        {
            type Output = Self;

            fn $method(self, other: T) -> Self::Output {
                let shape = self.shape().clone();
                let store = self.data().iter().map(|a| *a $op other).collect();
                let op = TensorOp::BinaryScalar(Box::new(self), other, BinaryOp::$trait);
                from_vec_with_op(false, op, shape, store)
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
                let op = TensorOp::BinaryScalar(Box::new(self.clone()), other, BinaryOp::$trait);
                from_vec_with_op(false, op, shape, store)
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
                let shape = self.shape().clone();
                let store = self.data().iter().zip(other.data().iter()).map(|(a, b)| *a $op *b).collect();
                let op = TensorOp::Binary(Box::new(self.clone()), Box::new(other), BinaryOp::$inner);

                *self = from_vec_with_op(false, op, shape, store);
            }
        }

        impl<'a, T> std::ops::$trait<&'a TensorBase<T>> for TensorBase<T>
        where
            T: Copy + std::ops::$inner<Output = T>,
        {
            fn $method(&mut self, other: &'a TensorBase<T>) {
                cmp!(ne: self.shape(), other.shape());
                let shape = self.shape().clone();
                let store = self.data().iter().zip(other.data().iter()).map(|(a, b)| *a $op *b).collect();
                let op = TensorOp::Binary(Box::new(self.clone()), Box::new(other.clone()), BinaryOp::$inner);

                *self = from_vec_with_op(false, op, shape, store);
            }
        }
    };

}

impl_arithmetic!(Add, add, +);
impl_arithmetic!(Div, div, /);
impl_arithmetic!(Mul, mul, *);
impl_arithmetic!(Sub, sub, -);

impl_assign_op!(AddAssign, add_assign, Add, +);
impl_assign_op!(DivAssign, div_assign, Div, /);
impl_assign_op!(MulAssign, mul_assign, Mul, *);
impl_assign_op!(SubAssign, sub_assign, Sub, -);
