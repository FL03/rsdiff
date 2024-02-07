/*
    Appellation: layout <mod>
    Contrib: FL03 <jo3mccain@icloud.com>
*/

use crate::shape::{IntoShape, Shape};

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Layout {
    pub(crate) offset: usize,
    pub(crate) shape: Shape,
    pub(crate) stride: Vec<usize>,
}

impl Layout {
    pub fn new(offset: usize, shape: Shape, stride: Vec<usize>) -> Self {
        Self {
            offset,
            shape,
            stride,
        }
    }

    pub fn contiguous(shape: impl IntoShape) -> Self {
        let shape = shape.into_shape();
        let stride = shape.stride_contiguous();
        Self {
            offset: 0,
            shape,
            stride,
        }
    }

    pub fn contiguous_with_offset(shape: impl IntoShape, offset: usize) -> Self {
        let shape = shape.into_shape();
        let stride = shape.stride_contiguous();
        Self {
            offset,
            shape,
            stride,
        }
    }

    pub fn elements(&self) -> usize {
        self.shape.elements()
    }

    pub fn offset(&self) -> usize {
        self.offset
    }

    pub fn shape(&self) -> &Shape {
        &self.shape
    }

    pub fn stride(&self) -> &Vec<usize> {
        &self.stride
    }
}
