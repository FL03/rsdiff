/*
    Appellation: specs <unary>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
use num::traits::Inv;
use num::Complex;

macro_rules! unary_op_trait {
    ($trait:ident, $method:ident) => {
        pub trait $trait {
            type Output;

            fn $method(self) -> Self::Output;
        }
    };
    (owned $trait:ident, $method:ident) => {
        pub trait $trait {
            type Output;

            fn $method(&self) -> Self::Output;
        }
    };
}

macro_rules! impl_unary_op {
    ($trait:ident, $method:ident, $t:ty) => {
        impl $trait for $t {
            type Output = $t;

            fn $method(self) -> Self::Output {
                <$t>::$method(self)
            }
        }
    };
    (generic $trait:ident, $method:ident, s => $s:tt, t => $t:tt) => {
        impl<S, T> $trait for S where S: $s, T: $t {
            type Output = T;

            fn $method(self) -> Self::Output {
                <$t>::$method(self)
            }
        }
    };
    ($trait:ident, $method:ident; [$($t:ty),*]) => {
        $(
            impl_unary_op!($trait, $method, $t);
        )*
    };
    ($trait:ident, $method:ident, $call:ident; $t:ty) => {
        impl $trait for $t {
            type Output = $t;

            fn $method(self) -> Self::Output {
                <$t>::$call(self)
            }
        }
    };
    (alts $trait:ident, $method:ident, $call:ident; [$($t:ty),*]) => {
        $(
            impl_unary_op!($trait, $method, $call; $t);
        )*
    };
}

unary_op_trait!(Abs, abs);
unary_op_trait!(Cos, cos);
unary_op_trait!(Cosh, cosh);
unary_op_trait!(Exp, exp);
unary_op_trait!(Ln, ln);
unary_op_trait!(Recip, recip);
unary_op_trait!(Sin, sin);
unary_op_trait!(Sinh, sinh);
unary_op_trait!(Sqrt, sqrt);
unary_op_trait!(Square, square);
unary_op_trait!(Tan, tan);
unary_op_trait!(Tanh, tanh);

impl<T> Abs for Complex<T>
where
    T: num::Float,
{
    type Output = T;

    fn abs(self) -> Self::Output {
        self.norm()
    }
}

impl<T> Recip for T
where
    T: Inv,
{
    type Output = <T as Inv>::Output;

    fn recip(self) -> Self::Output {
        self.inv()
    }
}

impl<T> Square for T
where
    T: Copy + std::ops::Mul<Self, Output = Self>,
{
    type Output = T;

    fn square(self) -> Self::Output {
        self * self
    }
}

impl_unary_op!(Abs, abs; [isize, i8, i16, i32, i64, i128, f32, f64]);
impl_unary_op!(Cos, cos; [f64, f32, Complex<f64>, Complex<f32>]);
impl_unary_op!(Cosh, cosh; [f64, f32, Complex<f64>, Complex<f32>]);
impl_unary_op!(Exp, exp; [f64, f32, Complex<f64>, Complex<f32>]);
impl_unary_op!(Ln, ln; [f64, f32, Complex<f64>, Complex<f32>]);
impl_unary_op!(Sin, sin; [f64, f32, Complex<f64>, Complex<f32>]);
impl_unary_op!(Sinh, sinh; [f64, f32, Complex<f64>, Complex<f32>]);
impl_unary_op!(Sqrt, sqrt; [f64, f32, Complex<f64>, Complex<f32>]);
impl_unary_op!(Tan, tan; [f64, f32, Complex<f64>, Complex<f32>]);
impl_unary_op!(Tanh, tanh; [f64, f32, Complex<f64>, Complex<f32>]);
