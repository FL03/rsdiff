/*
    Appellation: utils <mod>
    Contrib: FL03 <jo3mccain@icloud.com>
*/

macro_rules! unit_enum_constructor {
    ($(($variant:ident, $method:ident)),*) => {
        $(
            unit_enum_constructor!($variant, $method);
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
