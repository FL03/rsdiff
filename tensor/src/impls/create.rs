/*
    Appellation: create <impls>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
use crate::prelude::IntoShape;
use crate::tensor::*;
use num::traits::{FromPrimitive, NumAssign, One, Zero};

impl<T> TensorBase<T>
where
    T: Clone,
{
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
        let store = vec![value; shape.elements()];
        Self::from_vec(shape, store)
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
        // let steps = ((end - start) / step).ceil() as usize;
        let mut store = Vec::new();
        let mut value = start;
        while value < end {
            store.push(value);
            value += step;
        }
        from_vec((store.len(),), store)
    }

    /// Create a tensor within a range of values
    pub fn linstep(start: T, end: T, steps: usize) -> Self
    where
        T: FromPrimitive,
    {
        // let steps = ((end - start) / step).ceil() as usize;
        let step = (end - start) / T::from_usize(steps).unwrap();
        let mut store = Vec::with_capacity(steps);
        let mut value: T = start;
        for _ in 0..steps {
            store.push(value);
            value += step;
        }
        from_vec((store.len(),), store)
    }

    pub fn logstep(start: T, end: T, steps: usize) -> Self
    where
        T: num::Float,
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
        from_vec((store.len(),), store)
    }

    pub fn geomspace(start: T, end: T, steps: usize) -> Self
    where
        T: num::Float,
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
        from_vec((store.len(),), store)
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
        Self::ones(tensor.shape().clone())
    }
    /// Create a tensor, filled with ones, from the shape of the tensor
    pub fn ones_like(&self) -> Self {
        Self::ones(self.shape().clone())
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
        Self::zeros(tensor.shape().clone())
    }
    /// Create a tensor, filled with zeros, from the shape of the tensor
    pub fn zeros_like(&self) -> Self {
        Self::zeros(self.shape().clone())
    }
}
