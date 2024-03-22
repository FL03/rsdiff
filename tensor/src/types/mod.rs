/*
    Appellation: types <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/

pub mod id;
pub mod mode;
pub mod order;

pub(crate) mod prelude {
    pub use super::id::TensorId;
    pub use super::mode::TensorMode;
    pub use super::order::MajorOrder;
}
