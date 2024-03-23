/*
    Appellation: types <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/

pub mod id;
pub mod kinds;
pub mod order;

pub(crate) mod prelude {
    pub use super::id::TensorId;
    pub use super::kinds::TensorKind;
    pub use super::order::Order;
}
