/*
    Appellation: macros <mod>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
#![allow(unused_macros)]

///
#[macro_export]
macro_rules! nested {
    ($($i:ident in $iter:expr)=>* => $body:block ) => {
        nested!(@loop $body, $($i in $iter),*);
    };
    // The primary base case for iterators
    (@loop $body:block, $i:ident in $iter:expr) => {
        for $i in $iter $body
    };
    // An alternative base case
    (@loop $body:block, $i:ident in $iter:expr) => {
        for $i in $iter.into_iter() $body
    };
    // This is the recursive case. It will expand to a nested loop.
    (@loop $body:block, $i:ident in $iter:expr, $($tail:tt)*) => {
        for $i in $iter {
            nested!(@loop $body, $($tail)*);
        }
    };
}

macro_rules! impl_binary {
    (impl $($path:ident)::*, for $lhs:ident => $body:block) => {
        impl<T> $($path)::*<T> for $lhs where T: $($tr)* {
            type Output = $res;

            fn $method(self, rhs: $rhs) -> Self::Output $body
        }
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

macro_rules! evaluator {
    ($op:ident($($args:ident: $ty:ty),*) -> $output:ty $body:block) => {
        impl<P, $($args)*> Evaluator<P> for $op
        where
            P: $crate::ops::Params<Pattern = ($($arg),*)>,
        {
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
