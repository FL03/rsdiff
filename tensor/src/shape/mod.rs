/*
   Appellation: shapes <mod>
   Contrib: FL03 <jo3mccain@icloud.com>
*/
//! # Shapes
//!
//! This modules provides implements several useful primitives for working with
//! the shape of a [Tensor](crate::tensor::TensorBase).
pub use self::{axis::*, error::*, layout::Layout, rank::*, shape::Shape, stride::*};

pub(crate) mod axis;
pub(crate) mod error;
pub(crate) mod layout;
pub(crate) mod rank;
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

impl<'a> IntoShape for &'a Shape {
    fn into_shape(self) -> Shape {
        self.clone()
    }
}

pub(crate) mod prelude {
    pub use super::IntoShape;

    pub use super::axis::{Axis, IntoAxis};
    pub use super::dim::*;
    pub use super::error::*;
    pub use super::layout::Layout;
    pub use super::rank::{IntoRank, Rank};
    pub use super::shape::*;
    pub use super::stride::*;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_shape() {
        let mut shape = Shape::default();
        shape.extend([1, 1, 1]);
        assert_eq!(shape, Shape::new(vec![1, 1, 1]));
        assert_eq!(shape.size(), 1);
        assert_eq!(*shape.rank(), 3);
    }
}
