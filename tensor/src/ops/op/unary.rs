/*
   Appellation: unary <mod>
   Contrib: FL03 <jo3mccain@icloud.com>
*/
use crate::prelude::Scalar;

macro_rules! unary_op_trait {
    ($trait:ident, $method:ident) => {
        pub trait $trait {
            type Output;

            fn $method(self) -> Self::Output;
        }
    };
}

macro_rules! impl_unary_trait {
    ($trait:ident, $method:ident) => {
        impl<T> $trait for T
        where
            T: Scalar,
        {
            type Output = T;

            fn $method(self) -> Self::Output {
                <T>::$method(self)
            }
        }
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

impl<T> Abs for T
where
    T: num::Signed,
{
    type Output = T;

    fn abs(self) -> Self::Output {
        <T>::abs(&self)
    }
}

// impl<T> Cos for T
// where
//     T: Scalar,
// {
//     type Output = T;

//     fn cos(self) -> Self::Output {
//         <T>::cos(self)
//     }
// }

impl_unary_trait!(Cos, cos);
impl_unary_trait!(Cosh, cosh);
