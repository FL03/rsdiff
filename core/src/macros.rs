/*
    Appellation: macros <mod>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
#![allow(unused_macros)]

macro_rules! impl_binary {
    (custom $($args:tt),*) => {
        impl_binary!(@loop $(args),*);
    };
    ($lhs:ty, $rhs:ty, $res:ty: [$(( $op:ident, $method:ident, $e:expr)),*]) => {
        $(
            impl_binary!(@loop $lhs, $rhs, $res, $op, $method, $e);
        )*
    };
    ($lhs:ty, $rhs:ty, $res:ty, $op:ident, $method:ident, $e:expr) => {
        impl_binary!(@loop $lhs, $rhs, $res, $op, $method, $e);
    };

    (@loop $lhs:ty, $rhs:ty, $res:ty, $trait:path, $method:ident, $e:expr, where T: $($tr:tt)+*) => {
        impl<T> $trait<T> for $lhs where T: $($tr)* {
            type Output = $res;

            fn $method(self, rhs: $rhs) -> Self::Output {
                $e(self, rhs)
            }
        }
    };
    (@loop $lhs:ty, $rhs:ty, $res:ty, $op:ident, $method:ident, $e:expr) => {
        impl $op<$rhs> for $lhs {
            type Output = $res;

            fn $method(self, rhs: $rhs) -> Self::Output {
                $e(self, rhs)
            }
        }
    };
    (@lifetime $lhs:ty, $rhs:ty, $res:ty, $trait:path, $method:ident, $e:expr) => {
        impl<'a> $trait<$rhs> for $lhs {
            type Output = $res;

            fn $method(self, rhs: $rhs) -> Self::Output {
                $e(self, rhs)
            }
        }
    };
}

struct U(usize);

use core::ops::*;
impl_binary!(
    U, U, U: [
        (Add, add, | lhs: U, rhs: U | U(lhs.0 + rhs.0)),
        (Div, div, | lhs: U, rhs: U | U(lhs.0 / rhs.0)),
        (Mul, mul, | lhs: U, rhs: U | U(lhs.0 * rhs.0)),
        (Sub, sub, | lhs: U, rhs: U | U(lhs.0 - rhs.0))
    ]
);

macro_rules! impl_evaluator {
    (
        $op:ident,
        args:{$($args:ident: $ty:ty),*},
        output:$output:ty,
        call:$call:expr
    ) => {
        impl<$($args)*> Evaluator<$($args)*> for $op {
            type Output = $output

            fn eval(&self, ) -> Self::Output {
                $call($($args),*)
            }
        }
    };
}

macro_rules! operator {
    ($kind:ident: $($op:ident),*) => {
        $(
            operator!($op, $kind);
        )*
    };
    ($op:ident, $kind:ident) => {
        operator!($op, $kind, $op);
    };
    ($op:ident, $kind:ident, $name:ident) => {

        #[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
        #[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize,))]
        pub struct $op;

        impl $op {
            pub fn new() -> Self {
                Self
            }

            pub fn name(&self) -> &str {
                stringify!($name)
            }
        }

        impl core::fmt::Display for $op {
            fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                write!(f, "{}", self.name())
            }
        }

        impl $crate::ops::Operator for $op {

            fn kind(&self) -> $crate::ops::OpKind {
                OpKind::$kind
            }

            fn name(&self) -> &str {
                self.name()
            }
        }
    };

}
