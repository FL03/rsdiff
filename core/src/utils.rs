/*
    Appellation: utils <mod>
    Contrib: FL03 <jo3mccain@icloud.com>
*/

macro_rules! nested_constructor {
    ($variant:ident<$inner:ident>, $method:ident, [$($call:ident),*]) => {
        nested_constructor!(@loop $variant<$inner>, $method, [$($call),*]);
    };
    (@loop $variant:ident<$inner:ident>, $method:ident, [$($call:ident),*]) => {
        pub fn $method(inner:$inner) -> Self {
            Self::$variant(inner)
        }

        $(
            pub fn $call() -> Self {
                Self::$method($inner::$call())
            }
        )*

    };
}

macro_rules! variant_constructor {
    ($(($($rest:tt),*)),*) => {
        $(
            variant_constructor!(@loop $($rest),*);
        )*
    };
    ($(($variant:ident $($rest:tt),*, $method:ident)),*) => {
        $(
            variant_constructor!(@loop $variant $($rest),*, $method);
        )*
    };
    (@loop $variant:ident, $method:ident) => {
        pub fn $method() -> Self {
            Self::$variant
        }
    };

    (@loop $variant:ident($call:expr), $method:ident) => {
        pub fn $method() -> Self {
            Self::$variant($call())
        }
    };


}

#[allow(unused_macros)]
macro_rules! simple_enum_constructor {
    ($($n:tt)*) => {
        simple_enum_constructor!(@loop $($n)*);
    };
    (@loop $(($variant:ident {$($field:ident: $ty:ty),*}, $method:ident)),*) => {
        $(
            simple_enum_constructor!(@loop $variant {$($field: $ty),*}, $method);
        )*
    };

    (@loop $variant:ident {$($field:ident: $ty:ty),*}, $method:ident) => {
        pub fn $method($($field:$ty),*) -> Self {
            Self::$variant {
                $($field),*
            }
        }
    };
}
