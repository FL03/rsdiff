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
    (impl<$($t:ident),*> $name:ident($($args:tt)*) for $target:ident where $($arg:ident:$($bnd:tt)*),*) => {
        impl<$($t),*> core::fmt::$name for $target<$($t),*> where $($arg:$($bnd)*),* {
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
    ($lhs:ty, $rhs:ty, $res:ty: [$(($($path:ident)::*.$method:ident, $e:expr)),*]) => {
        $(
            impl_binary!(@loop $lhs, $rhs, $res, $($path)::*.$method, $e);
        )*
    };
    ($lhs:ty, $rhs:ty, $res:ty, $($path:ident)::*.$method:ident, $e:expr) => {
        impl_binary!(@loop $lhs, $rhs, $res, $($path)::*.$method, $e);
    };

    (@loop $lhs:ty, $rhs:ty, $res:ty, $($path:ident)::*.$method:ident, $e:expr, where T: $($tr:tt)+*) => {
        impl<T> $($path)::*<T> for $lhs where T: $($tr)* {
            type Output = $res;

            fn $method(self, rhs: $rhs) -> Self::Output {
                $e(self, rhs)
            }
        }
    };
    (@loop $lhs:ty, $rhs:ty, $res:ty, $($path:ident)::*.$method:ident, $e:expr) => {
        impl $($path)::*<$rhs> for $lhs {
            type Output = $res;

            fn $method(self, rhs: $rhs) -> Self::Output {
                $e(self, rhs)
            }
        }
    };
}

macro_rules! evaluator {
    ($op:ident($($args:ident),*) -> $output:ty where $($t:ident: $($rest:tt)*),* $body:block) => {
        impl<P, $($args)*> $crate::ops::Evaluate<P> for $op
        where
            P: $crate::ops::Params<Pattern = ($($arg),*)>,
            $($t: $($rest)*),*
        {
            type Output = $output;

            fn eval(&self, params: P) -> Self::Output $body
        }
    };
    ($op:ident<$($pat:tt)*>($($args:ident),*) -> $output:ty where $($t:ident: $($rest:tt)*),* $body:block) => {
        impl<P, $($args)*> $crate::ops::Evaluate<P> for $op
        where
            P: $crate::ops::Params<Pattern = $($pat)*>,
            $($t: $($rest)*),*
        {
            type Output = $output;

            fn eval(&self, params: P) -> Self::Output $body
        }
    };
    (nary $op:ident<Vec<T>> -> $output:ident where $($t:ident: $($rest:tt)*),* {$body:expr}) => {
        impl<P, T> $crate::ops::Evaluate<P> for $op
        where
            P: $crate::ops::Params<Pattern = Vec<T>>,
            $($t:$($rest)*),*
        {
            type Output = $output;

            fn eval(&self, params: P) -> Self::Output {
                $body(params)
            }
        }
    };
}

macro_rules! operand {
    ($($op:ident<$kind:ident>),*) => {
        $(
            operand!(@base $op<$kind>.$op);
        )*
    };
    ($($op:ident<$kind:ident>.$name:ident),*) => {
        $(
            operand!(@base $op<$kind>.$name);
        )*
    };
    (@base $op:ident<$kind:ident>.$name:ident) => {
        #[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
        #[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize,))]
        pub struct $op;

        impl $op {
            pub fn new() -> Self {
                Self
            }

            pub fn kind(&self) -> $crate::ops::OpKind::$kind {
                $crate::ops::OpKind::$kind
            }

            pub fn name(&self) -> &str {
                stringify!($name)
            }


        }

        operand!(@impl $op<$kind>.$name);
    };
    (@impl $op:ident<$kind:ident>.$name:ident) => {
        impl core::fmt::Display for $op {
            fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                write!(f, "{}", self.name())
            }
        }

        impl $crate::ops::Operand for $op {
            type Kind = $crate::ops::$kind;

            fn name(&self) -> &str {
                self.name()
            }

            fn optype(&self) -> Self::Kind {
                $crate::ops::$kind
            }
        }
    };

}

macro_rules! operations {
    ($group:ident<$kind:ident> {$($variant:ident($op:ident): $method:ident),*}) => {
        $(
            operation!($op<$kind>.$method);
        )*

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


            pub fn op(self) -> Box<dyn $crate::ops::Operator> {
                match self {
                    $(
                        $group::$variant(op) => Box::new(op),
                    )*
                }
            }
        }

        impl $crate::ops::Operand for $group {
            type Kind = $crate::ops::$kind;

            fn name(&self) -> &str {
                self.as_ref()
            }

            fn optype(&self) -> Self::Kind {
                $crate::ops::$kind
            }
        }
    };
}

macro_rules! operation {
    ($($op:ident<$kind:ident>),*) => {
        $(
            operation!(@base $op<$kind>.$op);
        )*
    };
    ($($op:ident<$kind:ident>.$name:ident),*) => {
        $(
            operation!(@base $op<$kind>.$name);
        )*
    };
    (@base $op:ident<$kind:ident>.$name:ident) => {
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

        operation!(@impl $op<$kind>.$name);
    };
    (@impl $op:ident<$kind:ident>.$name:ident) => {
        impl core::fmt::Display for $op {
            fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                write!(f, "{}", self.name())
            }
        }

        impl $crate::ops::Operand for $op {
            type Kind = $crate::ops::$kind;

            fn name(&self) -> &str {
                self.name()
            }

            fn optype(&self) -> Self::Kind {
                $crate::ops::$kind
            }
        }
    };

}

/* *************** Samples *************** */
struct U(usize);

impl_binary!(impl core::ops::Add.add(U, U) -> U { | lhs: U, rhs: U | U(lhs.0 + rhs.0) });
impl_binary!(
    U, U, U: [
        // (core::ops::Add, add, | lhs: U, rhs: U | U(lhs.0 + rhs.0)),
        (core::ops::Div.div, | lhs: U, rhs: U | U(lhs.0 / rhs.0)),
        (core::ops::Mul.mul, | lhs: U, rhs: U | U(lhs.0 * rhs.0)),
        (core::ops::Sub.sub, | lhs: U, rhs: U | U(lhs.0 - rhs.0))
    ]
);
