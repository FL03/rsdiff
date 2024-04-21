/*
    Appellation: macros <mod>
    Contrib: FL03 <jo3mccain@icloud.com>
*/

macro_rules! either {
    ($value:expr, $inner:pat => $result:expr) => {
        match $value {
            crate::iter::ElementsRepr::Slice($inner) => $result,
            crate::iter::ElementsRepr::Counted($inner) => $result,
        }
    };
}

macro_rules! either_mut {
    ($value:expr, $inner:ident => $result:expr) => {
        match $value {
            crate::iter::ElementsRepr::Slice(ref mut $inner) => $result,
            crate::iter::ElementsRepr::Counted(ref mut $inner) => $result,
        }
    };
}
