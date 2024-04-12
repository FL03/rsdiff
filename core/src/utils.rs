/*
    Appellation: utils <mod>
    Contrib: FL03 <jo3mccain@icloud.com>
*/

///
#[macro_export]
macro_rules! nested {
    ($(for $i:ident in $iter:expr),* => {$exp:expr} ) => {
        nested!(@loop $exp, $(for $i in $iter),*);
    };
    // This is the base case for the recursion.
    (@loop $exp:expr, for $i:ident in $iter:expr) => {
        for $i in $iter {
            $exp
        }
    };
    // This is the recursive case. It will expand to a nested loop.
    (@loop $exp:expr, for $i:ident in $iter:expr, $($tail:tt)*) => {
        for $i in $iter {
            nested!(@loop $exp, $($tail)*);
        }
    };
}

macro_rules! variant_constructor {
    ($(($variant:ident, $method:ident)),*) => {
        $(
            variant_constructor!($variant, $method);
        )*
    };
    ($variant:ident, $method:ident) => {
        pub fn $method() -> Self {
            Self::$variant
        }
    };
}

macro_rules! simple_enum_constructor {
    ($(($variant:ident, $method:ident, $new:expr)),*) => {
        $(
            simple_enum_constructor!($variant, $method, $new);
        )*
    };
    ($variant:ident, $method:ident, $new:expr) => {
        pub fn $method() -> Self {
            Self::$variant($new)
        }
    };
}
