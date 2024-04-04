/*
    Appellation: utils <mod>
    Contrib: FL03 <jo3mccain@icloud.com>
*/

macro_rules! enum_fn_constructor {
    ($(($variant:ident, $method:ident)),*) => {
        $(
            enum_fn_constructor!($variant, $method);
        )*
    };
    ($variant:ident, $method:ident) => {
        pub fn $method() -> Self {
            Self::$variant
        }
    };
}
