/*
    Appellation: macros <mod>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
#![allow(unused_macros)]

macro_rules! impl_binary {
    ($lhs:ty, $rhs:ty, $res:ty, $trait:path, $method:ident, $e:expr) => {
        impl_binary!(@base $lhs, $rhs, $res, $trait, $method, $e);
    };
    (generic $lhs:ty, $rhs:ty, $res:ty, $trait:path, $method:ident, $e:expr) => {
        impl<T> $trait<T> for $lhs {
            type Output = $res;

            fn $method(self, rhs: $rhs) -> Self::Output {
                $e(self, rhs)
            }
        }
    };
    (@base $lhs:ty, $rhs:ty, $res:ty, $trait:path, $method:ident, $e:expr) => {
        impl $trait<$rhs> for $lhs {
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

macro_rules! operator {
    ($kind:ident: $($op:ident),*) => {
        $(
            operator!($op, $kind);
        )*
    };
    ($op:ident, $kind:ident) => {

        #[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
        #[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize,))]
        pub struct $op;

        impl $op {
            pub fn new() -> Self {
                Self
            }

            pub fn name(&self) -> &str {
                stringify!($op)
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
