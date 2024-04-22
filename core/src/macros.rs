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

macro_rules! impl_fmt {
    ($target:ident, $name:ident($($args:tt)*)) => {
        impl core::fmt::$name for $target {
            fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                write!(f, $($args)*)
            }
        }
    };
    ($target:ident<$($t:ident),*>, $name:ident($($args:tt)*)) => {
        impl<$($t),*> core::fmt::$name for $target<$($t),*> {
            fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                write!(f, $($args)*)
            }
        }
    };
}

macro_rules! impl_binary {
    (impl $($path:ident)::*.$call:ident($lhs:ident, $rhs:ident) -> $res:ident {$body:expr}) => {
        impl $($path)::*<$rhs> for $lhs {
            type Output = $res;

            fn $call(self, rhs: $rhs) -> Self::Output {
                $body(self, rhs)
            }
        }
    };
    (impl $($path:ident)::*<$rhs:ident>.$call:ident for $lhs:ident -> $res:ident where $($t:ident:$($bnd:tt),*),* {$body:expr}) => {
        impl<$($t),*> $($path)::*<$rhs> for $lhs where $($t:$($bnd),*)* {
            type Output = $res;

            fn $method(self, rhs: $rhs) -> Self::Output {
                $body(self, rhs)
            }
        }
    };
    ($lhs:ty, $rhs:ty, $res:ty: [$(($($path:ident)::*, $method:ident, $e:expr)),*]) => {
        $(
            impl_binary!(@loop $lhs, $rhs, $res, $($path)::*, $method, $e);
        )*
    };
    ($lhs:ty, $rhs:ty, $res:ty, $($path:ident)::*, $method:ident, $e:expr) => {
        impl_binary!(@loop $lhs, $rhs, $res, $($path)::*, $method, $e);
    };

    (@loop $lhs:ty, $rhs:ty, $res:ty, $($path:ident)::*, $method:ident, $e:expr, where T: $($tr:tt)+*) => {
        impl<T> $($path)::*<T> for $lhs where T: $($tr)* {
            type Output = $res;

            fn $method(self, rhs: $rhs) -> Self::Output {
                $e(self, rhs)
            }
        }
    };
    (@loop $lhs:ty, $rhs:ty, $res:ty, $($path:ident)::*, $method:ident, $e:expr) => {
        impl $($path)::*<$rhs> for $lhs {
            type Output = $res;

            fn $method(self, rhs: $rhs) -> Self::Output {
                $e(self, rhs)
            }
        }
    };
}

struct U(usize);

impl_binary!(impl core::ops::Add.add(U, U) -> U { | lhs: U, rhs: U | U(lhs.0 + rhs.0) });
impl_binary!(
    U, U, U: [
        // (core::ops::Add, add, | lhs: U, rhs: U | U(lhs.0 + rhs.0)),
        (core::ops::Div, div, | lhs: U, rhs: U | U(lhs.0 / rhs.0)),
        (core::ops::Mul, mul, | lhs: U, rhs: U | U(lhs.0 * rhs.0)),
        (core::ops::Sub, sub, | lhs: U, rhs: U | U(lhs.0 - rhs.0))
    ]
);

macro_rules! operator_group {
    ($group:ident<$kind:ident> {$($variant:ident($op:ident): $method:ident),*}) => {
        #[derive(
            Clone,
            Copy,
            Debug,
            Eq,
            Hash,
            Ord,
            PartialEq,
            PartialOrd,
            strum::AsRefStr,
            strum::Display,
            strum::EnumCount,
            strum::EnumIs,
            strum::EnumIter,
            strum::EnumString,
            strum::VariantNames,
        )]
        #[cfg_attr(
            feature = "serde",
            derive(serde::Deserialize, serde::Serialize,),
            serde(rename_all = "snake_case", untagged),
        )]
        #[repr(C)]
        #[strum(serialize_all = "snake_case")]
        pub enum $group {
            $(
                $variant($op),
            )*
        }

        impl $group {
            $(
                pub fn $method() -> Self {
                    Self::$variant($op::new())
                }
            )*

            pub fn name(&self) -> &str {
                self.as_ref()
            }


            pub fn op(self) -> Box<dyn Operator> {
                match self {
                    $(
                        $group::$variant(op) => Box::new(op),
                    )*
                }
            }
        }

        impl Operator for $group {
            fn kind(&self) -> OpKind {
                OpKind::$kind
            }

            fn name(&self) -> &str {
                self.as_ref()
            }
        }
    };
}

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
    ($(($op:ident<$kind:ident>, $name:ident)),*) => {
        $(
            operator!(@impl $op<$kind>, $name);
        )*
    };
    ($($op:ident<$kind:ident>),*) => {
        $(
            operator!(@impl $op<$kind>, $op);
        )*
    };
    ($kind:ident: $($op:ident),*) => {
        operator!($(op<$kind>),*);
    };
    ($kind:ident: $(($op:ident, $name:ident)),*) => {
        operator!($(($op<$kind>, $name)),*);
    };
    ($op:ident<$kind:ident>) => {
        operator!(@impl $op<$kind>, $op);
    };
    ($op:ident<$kind:ident>, $name:ident) => {
        operator!(@impl $op<$kind>, $name);
    };
    (@impl $op:ident<$kind:ident>, $name:ident) => {

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
