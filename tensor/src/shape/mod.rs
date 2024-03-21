/*
   Appellation: shapes <mod>
   Contrib: FL03 <jo3mccain@icloud.com>
*/
//! # Shapes
pub use self::{dimension::*, rank::*, shape::*, stride::*};

pub(crate) mod dimension;
pub(crate) mod rank;
pub(crate) mod shape;
pub(crate) mod stride;

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
    pub use super::dimension::*;
    pub use super::rank::*;
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
        assert_eq!(shape.elements(), 1);
        assert_eq!(*shape.rank(), 3);
    }
}
