/*
    Appellation: arith <mod>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
use crate::prelude::TensorOp;
use crate::tensor::{from_vec_with_op, TensorBase};
use acme::ops::binary::BinaryOp;
use num::traits::Pow;

macro_rules! cmp {
    (ne: $lhs:expr, $rhs:expr) => {
        if $lhs != $rhs {
            panic!("Shape Mismatch: {:?} != {:?}", $lhs, $rhs);
        }
    };
}

impl<T> Pow<T> for TensorBase<T>
where
    T: Copy + Pow<T, Output = T>,
{
    type Output = Self;

    fn pow(self, exp: T) -> Self::Output {
        let shape = self.shape().clone();
        let store = self.data().iter().map(|a| a.pow(exp)).collect();
        let op = TensorOp::binary_scalar(self, exp, BinaryOp::Pow);
        from_vec_with_op(false, op, shape, store)
    }
}

impl<'a, T> Pow<T> for &'a TensorBase<T>
where
    T: Copy + Pow<T, Output = T>,
{
    type Output = TensorBase<T>;

    fn pow(self, exp: T) -> Self::Output {
        let shape = self.shape().clone();
        let store = self.data().iter().map(|a| a.pow(exp)).collect();
        let op = TensorOp::binary_scalar(self.clone(), exp, BinaryOp::Pow);
        from_vec_with_op(false, op, shape, store)
    }
}

macro_rules! impl_arithmetic {
    (op: $trait:ident, $method:ident, $op:tt) => {
        impl_scalar_arith!($trait, $method, $op);

        impl<T> std::ops::$trait for TensorBase<T>
        where
            T: Copy + std::ops::$trait<Output = T>,
        {
            type Output = Self;

            fn $method(self, other: Self) -> Self::Output {
                cmp!(ne: self.shape(), other.shape());
                let shape = self.shape().clone();
                let store = self.data().iter().zip(other.data().iter()).map(|(a, b)| *a $op *b).collect();
                let op = TensorOp::binary(self, other, BinaryOp::$trait);
                from_vec_with_op(false, op, shape, store)
            }
        }

        impl<'a, T> std::ops::$trait<&'a TensorBase<T>> for TensorBase<T>
        where
            T: Copy + std::ops::$trait<Output = T>,
        {
            type Output = TensorBase<T>;

            fn $method(self, other: &'a TensorBase<T>) -> Self::Output {
                if self.shape() != other.shape() {
                    panic!("shapes must be equal");
                }
                let shape = self.shape().clone();
                let store = self.data().iter().zip(other.data().iter()).map(|(a, b)| *a $op *b).collect();
                let op = TensorOp::binary(self, other.clone(), BinaryOp::$trait);
                from_vec_with_op(false, op, shape, store)
            }
        }

        impl<'a, T> std::ops::$trait<TensorBase<T>> for &'a TensorBase<T>
        where
            T: Copy + std::ops::$trait<Output = T>,
        {
            type Output = TensorBase<T>;

            fn $method(self, other: TensorBase<T>) -> Self::Output {
                if self.shape() != other.shape() {
                    panic!("shapes must be equal");
                }
                let shape = self.shape().clone();
                let store = self.data().iter().zip(other.data().iter()).map(|(a, b)| *a $op *b).collect();
                let op = TensorOp::binary(self.clone(), other, BinaryOp::$trait);
                from_vec_with_op(false, op, shape, store)
            }
        }

        impl<'a, 'b, T> std::ops::$trait<&'b TensorBase<T>> for &'a TensorBase<T>
        where
            T: Copy + std::ops::$trait<Output = T>,
        {
            type Output = TensorBase<T>;

            fn $method(self, other: &'b TensorBase<T>) -> Self::Output {
                if self.shape() != other.shape() {
                    panic!("shapes must be equal");
                }
                let shape = self.shape().clone();
                let store = self.data().iter().zip(other.data().iter()).map(|(a, b)| *a $op *b).collect();
                let op = TensorOp::binary(self.clone(), other.clone(), BinaryOp::$trait);
                from_vec_with_op(false, op, shape, store)
            }
        }
    };
    ($(($trait:ident, $method:ident, $op:tt)),*) => {
        $( impl_arithmetic!(op: $trait, $method, $op); )*
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
                let op = TensorOp::binary_scalar(self, other, BinaryOp::$trait);
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
                let op = TensorOp::binary_scalar(self.clone(), other, BinaryOp::$trait);
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
                let op = TensorOp::binary(self.clone(), other, BinaryOp::$inner);

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
                let op = TensorOp::binary(self.clone(), other.clone(), BinaryOp::$inner);

                *self = from_vec_with_op(false, op, shape, store);
            }
        }
    };

}

impl_arithmetic!((Add, add, +), (Div, div, /), (Mul, mul, *), (Rem, rem, %), (Sub, sub, -));

impl_assign_op!(AddAssign, add_assign, Add, +);
impl_assign_op!(DivAssign, div_assign, Div, /);
impl_assign_op!(MulAssign, mul_assign, Mul, *);
impl_assign_op!(RemAssign, rem_assign, Rem, %);
impl_assign_op!(SubAssign, sub_assign, Sub, -);
