/*
    Appellation: stores <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
pub use self::specs::*;

pub(crate) mod specs;

pub(crate) mod prelude {
    pub use super::specs::*;
}
