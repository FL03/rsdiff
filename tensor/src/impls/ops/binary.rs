/*
    Appellation: arith <mod>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
use crate::prelude::{Scalar, TensorExpr};
use crate::tensor::{from_vec_with_op, TensorBase};
use acme::ops::binary::BinaryOp;
use core::ops;
use num::traits::float::{Float, FloatCore};
use num::traits::Pow;

#[allow(dead_code)]
pub(crate) fn broadcast_scalar_op<F, T>(
    lhs: &TensorBase<T>,
    rhs: &TensorBase<T>,
    op: BinaryOp,
    f: F,
) -> TensorBase<T>
where
    F: Fn(T, T) -> T,
    T: Copy + Default,
{
    let mut lhs = lhs.clone();
    let mut rhs = rhs.clone();
    if lhs.is_scalar() {
        lhs = lhs.broadcast(rhs.shape());
    }
    if rhs.is_scalar() {
        rhs = rhs.broadcast(lhs.shape());
    }
    let shape = lhs.shape().clone();
    let store = lhs
        .data()
        .iter()
        .zip(rhs.data().iter())
        .map(|(a, b)| f(*a, *b))
        .collect();
    let op = TensorExpr::binary(lhs, rhs, op);
    from_vec_with_op(false, op, shape, store)
}

fn check_shapes_or_scalar<T>(lhs: &TensorBase<T>, rhs: &TensorBase<T>)
where
    T: Clone + Default,
{
    let is_scalar = lhs.is_scalar() || rhs.is_scalar();
    debug_assert!(
        is_scalar || lhs.shape() == rhs.shape(),
        "Shape Mismatch: {:?} != {:?}",
        lhs.shape(),
        rhs.shape()
    );
}

macro_rules! check {
    (ne: $lhs:expr, $rhs:expr) => {
        if $lhs != $rhs {
            panic!("Shape Mismatch: {:?} != {:?}", $lhs, $rhs);
        }
    };
}

impl<T> TensorBase<T>
where
    T: Scalar,
{
    pub fn apply_binary(&self, other: &Self, op: BinaryOp) -> Self {
        check_shapes_or_scalar(self, other);
        let shape = self.shape();
        let store = self
            .data()
            .iter()
            .zip(other.data().iter())
            .map(|(a, b)| *a + *b)
            .collect();
        let op = TensorExpr::binary(self.clone(), other.clone(), op);
        from_vec_with_op(false, op, shape, store)
    }

    pub fn apply_binaryf<F>(&self, other: &Self, op: BinaryOp, f: F) -> Self
    where
        F: Fn(T, T) -> T,
    {
        check_shapes_or_scalar(self, other);
        let shape = self.shape();
        let store = self
            .data()
            .iter()
            .zip(other.data().iter())
            .map(|(a, b)| f(*a, *b))
            .collect();
        let op = TensorExpr::binary(self.clone(), other.clone(), op);
        from_vec_with_op(false, op, shape, store)
    }
}

impl<T> TensorBase<T> {
    pub fn pow(&self, exp: T) -> Self
    where
        T: Copy + Pow<T, Output = T>,
    {
        let shape = self.shape();
        let store = self.data().iter().copied().map(|a| a.pow(exp)).collect();
        let op = TensorExpr::binary_scalar(self.clone(), exp, BinaryOp::Pow);
        from_vec_with_op(false, op, shape, store)
    }

    pub fn powf(&self, exp: T) -> Self
    where
        T: Float,
    {
        let shape = self.shape();
        let store = self.data().iter().copied().map(|a| a.powf(exp)).collect();
        let op = TensorExpr::binary_scalar(self.clone(), exp, BinaryOp::Pow);
        from_vec_with_op(false, op, shape, store)
    }

    pub fn powi(&self, exp: i32) -> Self
    where
        T: FloatCore,
    {
        let shape = self.shape();
        let store = self.data().iter().copied().map(|a| a.powi(exp)).collect();
        let op = TensorExpr::binary_scalar(self.clone(), T::from(exp).unwrap(), BinaryOp::Pow);
        from_vec_with_op(false, op, shape, store)
    }
}

// impl<T> TensorBase<T> where T: ComplexFloat<Real = T> + Scalar<Complex = Complex<T>, Real = T> {

//     pub fn powc(&self, exp: <T as Scalar>::Complex) -> TensorBase<<T as Scalar>::Complex> {
//         let shape = self.shape();
//         let store = self.data().iter().copied().map(|a| Scalar::powc(a, exp)).collect();
//         let op = TensorExpr::binary_scalar_c(self.clone(), exp, BinaryOp::Pow);
//         TensorBase {
//             id: TensorId::new(),
//             data: store,
//             kind: TensorKind::default(),
//             layout: Layout::contiguous(shape),
//             op: BackpropOp::new(op)
//         }
//     }
// }

impl<T> Pow<T> for TensorBase<T>
where
    T: Copy + Pow<T, Output = T>,
{
    type Output = Self;

    fn pow(self, exp: T) -> Self::Output {
        let shape = self.shape().clone();
        let store = self.data().iter().map(|a| a.pow(exp)).collect();
        let op = TensorExpr::binary_scalar(self, exp, BinaryOp::Pow);
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
        let op = TensorExpr::binary_scalar(self.clone(), exp, BinaryOp::Pow);
        from_vec_with_op(false, op, shape, store)
    }
}

macro_rules! impl_binary_op {
    ($(($trait:ident, $method:ident, $op:tt)),*) => {
        $( impl_binary_op!($trait, $method, $op); )*
    };
    ($trait:ident, $method:ident, $op:tt) => {
        impl_binary_op!(scalar: $trait, $method, $op);
        impl_binary_op!(tensor: $trait, $method, $op);
    };
    (scalar: $trait:ident, $method:ident, $op:tt) => {
        
        impl<T> ops::$trait<T> for TensorBase<T>
        where
            T: Copy + ops::$trait<Output = T>,
        {
            type Output = Self;

            fn $method(self, other: T) -> Self::Output {
                let shape = self.shape().clone();
                let store = self.data().iter().map(|a| *a $op other).collect();
                let op = TensorExpr::binary_scalar(self, other, BinaryOp::$method());
                from_vec_with_op(false, op, shape, store)
            }
        }

        impl<'a, T> ops::$trait<T> for &'a TensorBase<T>
        where
            T: Copy + ops::$trait<Output = T>,
        {
            type Output = TensorBase<T>;

            fn $method(self, other: T) -> Self::Output {
                let shape = self.shape().clone();
                let store = self.data().iter().map(|a| *a $op other).collect();
                let op = TensorExpr::binary_scalar(self.clone(), other, BinaryOp::$method());
                from_vec_with_op(false, op, shape, store)
            }
        }
    };
    (tensor: $trait:ident, $method:ident, $op:tt) => {
        impl<T> ops::$trait for TensorBase<T>
        where
            T: Copy + ops::$trait<Output = T>,
        {
            type Output = Self;

            fn $method(self, other: Self) -> Self::Output {
                check!(ne: self.shape(), other.shape());
                let shape = self.shape().clone();
                let store = self.data().iter().zip(other.data().iter()).map(|(a, b)| *a $op *b).collect();
                let op = TensorExpr::binary(self, other, BinaryOp::$method());
                from_vec_with_op(false, op, shape, store)
            }
        }

        impl<'a, T> ops::$trait<&'a TensorBase<T>> for TensorBase<T>
        where
            T: Copy + ops::$trait<Output = T>,
        {
            type Output = TensorBase<T>;

            fn $method(self, other: &'a TensorBase<T>) -> Self::Output {
                if self.shape() != other.shape() {
                    panic!("shapes must be equal");
                }
                let shape = self.shape().clone();
                let store = self.data().iter().zip(other.data().iter()).map(|(a, b)| *a $op *b).collect();
                let op = TensorExpr::binary(self, other.clone(), BinaryOp::$method());
                from_vec_with_op(false, op, shape, store)
            }
        }

        impl<'a, T> ops::$trait<TensorBase<T>> for &'a TensorBase<T>
        where
            T: Copy + ops::$trait<Output = T>,
        {
            type Output = TensorBase<T>;

            fn $method(self, other: TensorBase<T>) -> Self::Output {
                if self.shape() != other.shape() {
                    panic!("shapes must be equal");
                }
                let shape = self.shape().clone();
                let store = self.data().iter().zip(other.data().iter()).map(|(a, b)| *a $op *b).collect();
                let op = TensorExpr::binary(self.clone(), other, BinaryOp::$method());
                from_vec_with_op(false, op, shape, store)
            }
        }

        impl<'a, 'b, T> ops::$trait<&'b TensorBase<T>> for &'a TensorBase<T>
        where
            T: Copy + ops::$trait<Output = T>,
        {
            type Output = TensorBase<T>;

            fn $method(self, other: &'b TensorBase<T>) -> Self::Output {
                if self.shape() != other.shape() {
                    panic!("shapes must be equal");
                }
                let shape = self.shape().clone();
                let store = self.data().iter().zip(other.data().iter()).map(|(a, b)| *a $op *b).collect();
                let op = TensorExpr::binary(self.clone(), other.clone(), BinaryOp::$method());
                from_vec_with_op(false, op, shape, store)
            }
        }
    };

}

macro_rules! impl_assign_op {
    ($trait:ident, $method:ident, $constructor:ident, $inner:ident, $op:tt) => {
        impl<T> core::ops::$trait for TensorBase<T>
        where
            T: Copy + core::ops::$inner<T, Output = T>,
        {
            fn $method(&mut self, other: Self) {
                check!(ne: self.shape(), other.shape());
                let shape = self.shape().clone();
                let store = self.data().iter().zip(other.data().iter()).map(|(a, b)| *a $op *b).collect();
                let op = TensorExpr::binary(self.clone(), other, BinaryOp::$constructor());

                *self = from_vec_with_op(false, op, shape, store);
            }
        }

        impl<'a, T> core::ops::$trait<&'a TensorBase<T>> for TensorBase<T>
        where
            T: Copy + core::ops::$inner<Output = T>,
        {
            fn $method(&mut self, other: &'a TensorBase<T>) {
                check!(ne: self.shape(), other.shape());
                let shape = self.shape().clone();
                let store = self.data().iter().zip(other.data().iter()).map(|(a, b)| *a $op *b).collect();
                let op = TensorExpr::binary(self.clone(), other.clone(), BinaryOp::$constructor());

                *self = from_vec_with_op(false, op, shape, store);
            }
        }
    };

}

macro_rules! impl_binary_method {
    ($method:ident, $f:expr) => {
        pub fn $method(&self, other: &Self) -> Self {
            $f(self, other)
        }

    };
    (scalar: $variant:tt, $method:ident, $op:tt) => {
        pub fn $method(&self, other: T) -> Self {
            let shape = self.shape();
            let store = self.data().iter().map(| elem | *elem $op other).collect();
            let op = TensorExpr::binary_scalar(self.clone(), other, BinaryOp::$variant());
            from_vec_with_op(false, op, shape, store)
        }

    };
    (tensor: $method:ident, $op:tt) => {
        pub fn $method(&self, other: &Self) -> Self {
            check!(ne: self.shape(), other.shape());
            let shape = self.shape();
            let store = self.data().iter().zip(other.data().iter()).map(|(a, b)| *a $op *b).collect();
            let op = TensorExpr::binary(self.clone(), other.clone(), BinaryOp::$method());
            from_vec_with_op(false, op, shape, store)
        }

    };
}

impl_binary_op!((Add, add, +), (Div, div, /), (Mul, mul, *), (Rem, rem, %), (Sub, sub, -));

impl_assign_op!(AddAssign, add_assign, add, Add, +);
impl_assign_op!(DivAssign, div_assign, div, Div, /);
impl_assign_op!(MulAssign, mul_assign, mul, Mul, *);
impl_assign_op!(RemAssign, rem_assign, rem, Rem, %);
impl_assign_op!(SubAssign, sub_assign, sub, Sub, -);

impl<T> TensorBase<T>
where
    T: Scalar,
{
    impl_binary_method!(tensor: add, +);
    impl_binary_method!(scalar: add, add_scalar, +);
}
