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
