/*
   Appellation: shapes <mod>
   Contrib: FL03 <jo3mccain@icloud.com>
*/
//! # Shapes
//!
//!
pub use self::{error::*, shape::*, stride::*};

pub(crate) mod error;
pub(crate) mod shape;
pub(crate) mod stride;

pub mod dim;

pub trait IntoShape {
    fn into_shape(self) -> Shape;
}

impl<S> IntoShape for S
where
    S: Into<Shape>,
{
    fn into_shape(self) -> Shape {
        self.into()
    }
}

pub(crate) mod prelude {
    pub use super::dim::*;
    pub use super::error::*;
    pub use super::shape::*;
    pub use super::stride::*;
    pub use super::IntoShape;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_shape() {
        let mut shape = Shape::default();
        shape.extend([1, 1, 1]);
        assert_eq!(shape, Shape::new(vec![1, 1, 1]));
        assert_eq!(shape.elements(), 1);
        assert_eq!(*shape.rank(), 3);
    }
}
