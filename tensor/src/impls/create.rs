/*
    Appellation: create <impls>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
use crate::prelude::IntoShape;
use crate::tensor::{from_vec_with_kind, TensorBase};
use num::traits::real::Real;
use num::traits::{FromPrimitive, NumAssign, One, Zero};

impl<T> TensorBase<T>
where
    T: Clone,
{
    /// Create a new tensor, whose elements are set to their default value
    /// from the current shape.
    pub fn default_like(&self) -> Self
    where
        T: Default,
    {
        Self::fill(self.shape(), T::default())
    }
    /// Create an empty tensor from the given shape
    pub fn empty(shape: impl IntoShape) -> Self
    where
        T: Default,
    {
        Self::fill(shape, T::default())
    }
    /// Create a tensor, from the given shape, filled with the given value
    pub fn fill(shape: impl IntoShape, value: T) -> Self {
        let shape = shape.into_shape();
        let store = vec![value; shape.size()];
        from_vec_with_kind(false, shape, store)
    }
    /// Create a tensor, filled with some value, from the current shape
    pub fn fill_like(&self, value: T) -> Self {
        Self::fill(self.shape(), value)
    }
}

impl<T> TensorBase<T>
where
    T: Copy + NumAssign + PartialOrd,
{
    /// Create a tensor within a range of values
    pub fn arange(start: T, end: T, step: T) -> Self {
        if T::is_zero(&step) {
            panic!("step must be non-zero");
        }
        let mut store = Vec::new();
        let mut value = start;
        while value < end {
            store.push(value);
            value += step;
        }
        Self::from_vec(store)
    }
    /// Create an identity matrix of a certain size
    pub fn eye(size: usize) -> Self {
        let mut store = Vec::with_capacity(size * size);
        for i in 0..size {
            for j in 0..size {
                store.push(if i == j { T::one() } else { T::zero() });
            }
        }
        Self::from_shape_vec((size, size), store)
    }
    /// Create a tensor with a certain number of elements, evenly spaced
    /// between the provided start and end values
    pub fn linspace(start: T, end: T, steps: usize) -> Self
    where
        T: FromPrimitive,
    {
        let step = (end - start) / T::from_usize(steps).unwrap();
        Self::arange(start, end, step)
    }

    pub fn logspace(start: T, end: T, steps: usize) -> Self
    where
        T: Real,
    {
        let start = start.log2();
        let end = end.log2();
        let step = (end - start) / T::from(steps).unwrap();
        let mut store = Vec::with_capacity(steps);
        let mut value: T = start;
        for _ in 0..steps {
            store.push(value.exp2());
            value += step;
        }
        from_vec_with_kind(false, (store.len(),), store)
    }

    pub fn geomspace(start: T, end: T, steps: usize) -> Self
    where
        T: Real,
    {
        let start = start.log10();
        let end = end.log10();
        let step = (end - start) / T::from(steps).unwrap();
        let mut store = Vec::with_capacity(steps);
        let mut value: T = start;
        for _ in 0..steps {
            store.push(value.exp());
            value += step;
        }
        from_vec_with_kind(false, (store.len(),), store)
    }
}

impl<T> TensorBase<T>
where
    T: Clone + One,
{
    /// Create a tensor, filled with ones, from the given shape
    pub fn ones(shape: impl IntoShape) -> Self {
        Self::fill(shape, T::one())
    }
    /// Create a tensor, filled with ones, from the shape of another tensor
    pub fn ones_from(tensor: &TensorBase<T>) -> Self {
        Self::ones(tensor.shape())
    }
    /// Create a tensor, filled with ones, from the shape of the tensor
    pub fn ones_like(&self) -> Self {
        Self::ones(self.shape())
    }
}

impl<T> TensorBase<T>
where
    T: Clone + Zero,
{
    /// Create a tensor, filled with zeros, from the given shape
    pub fn zeros(shape: impl IntoShape) -> Self {
        Self::fill(shape, T::zero())
    }
    /// Create a tensor, filled with zeros, from the shape of another tensor
    pub fn zeros_from(tensor: &TensorBase<T>) -> Self {
        Self::zeros(tensor.shape())
    }
    /// Create a tensor, filled with zeros, from the shape of the tensor
    pub fn zeros_like(&self) -> Self {
        Self::zeros(self.shape())
    }
}
