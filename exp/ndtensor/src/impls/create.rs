/*
    Appellation: create <impls>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
use crate::prelude::TensorError;
use crate::TensorBase;
use ndarray::*;
use num::{One, Zero};

macro_rules! map_method {
    // ($method:ident) => {
    //     pub fn $method(&self) -> Self {
    //         new!(self.data.$method())
    //     }
    // };
    (a $method:ident$($rest:tt),*) => {
        map_method!(@impl $method$($rest)*);
    };
    ($method:ident($($field:ident:$ty:ty),*) where $($tb:ident: $($ext:ident)+),*) => {
        map_method!(@impl $method($($field:$ty),*) where $($tb: $($ext)+),*);
    };
    ($method:ident($($field:ident:$ty:ty),*) where $($tb:ident: $($ext:ident)+),* $($rest:tt),*) => {
        map_method!(@impl $method($($field:$ty),*) where $($tb: $($ext)+),*$($rest)*);
    };
    ($method:ident($($field:ident:$ty:ty),*) where $($tb:ident: $($ext:ident)+),* => $($res:ident),*) => {
        map_method!(@impl $method($($field:$ty),*) where $($tb: $($ext)+),* => $($res:ident),*);
    };
    ($method:ident<$($t:ident),*>($($field:ident:$ty:ty),*) where $($tb:ident: $($ext:ident)++),*) => {
        map_method!(@impl $method<$($t),*>($($field:$ty),*) where $($tb: $($ext)++),*);
    };
    (@impl $method:ident($($field:ident:$ty:ty),*) where $($tb:ident: $($ext:ident)+),* => $($res:ident),*) => {
        pub fn $method($($field:$ty),*) -> Result<$res, TensorError>
        where
            $($tb: $($ext)++),*
        {
            new!(ArrayBase::$method($($field),*)?)
        }
    };
    (@impl $method:ident($($field:ident:$ty:ty),*) where $($tb:ident: $($ext:ident)+),*) => {
        pub fn $method($($field:$ty),*) -> Self
        where
            $($tb: $($ext)++),*
        {
            new!(ArrayBase::$method($($field),*))
        }
    };
    (@impl $method:ident<$($t:ident),*>($($field:ident:$ty:ty),*) where $($tb:ident: $($ext:ident)++),*) => {
        pub fn $method<$($t),*>($($field:$ty),*) -> Self
        where
            $($tb: $($ext)++),*
        {
            new!(self.data.$method($($field),*))
        }
    };
}

impl<A, S, D> TensorBase<S, D>
where
    D: Dimension,
    S: RawData<Elem = A>,
{
    pub fn arange(start: A, end: A, step: A) -> TensorBase<S, Ix1>
    where
        A: Clone + num::Float,
        S: DataOwned,
    {
        new!(ArrayBase::range(start, end, step))
    }

    pub fn from_arr(data: ArrayBase<S, D>) -> Self {
        new!(data)
    }

    pub fn from_shape_vec(shape: D, data: Vec<S::Elem>) -> Result<Self, TensorError>
    where
        S: DataOwned,
    {
        let data = ArrayBase::from_shape_vec(shape, data)?;
        Ok(new!(data))
    }

    pub fn try_from_arr<D2>(data: ArrayBase<S, D2>) -> Result<Self, TensorError>
    where
        D2: Dimension,
    {
        let tensor = Self::from_arr(data.into_dimensionality::<D>()?);
        Ok(tensor)
    }

    // map_method!(from_shape_vec(shape: D, data: Vec<S::Elem>) where S: DataOwned => Self);

    map_method!(from_elem(shape: D, elem: A) where A: Clone, S: DataOwned);

    pub fn fill(shape: D, elem: A) -> Self
    where
        A: Clone,
        S: DataOwned,
    {
        new!(ArrayBase::from_elem(shape, elem))
    }

    pub fn linspace(start: A, end: A, num: usize) -> TensorBase<S, Ix1>
    where
        A: Clone + num::Float,
        S: DataOwned,
    {
        new!(ArrayBase::linspace(start, end, num))
    }

    pub fn linshape(shape: impl IntoDimension<Dim = D>) -> Result<TensorBase<S, D>, ShapeError>
    where
        A: Clone + num::Float,
        S: DataOwned,
    {
        let dim = shape.into_dimension();
        let n = dim.ndim();
        Self::linspace(A::zero(), A::from(n).unwrap(), n - 1).into_shape(dim)
    }

    pub fn ones<Sh>(shape: Sh) -> Self
    where
        A: Clone + One,
        S: DataOwned,
        Sh: ShapeBuilder<Dim = D>,
    {
        new!(ArrayBase::ones(shape))
    }

    pub fn ones_like(&self) -> Self
    where
        A: Clone + One,
        S: DataOwned,
    {
        Self::ones(self.dim())
    }

    pub fn zeros<Sh>(shape: Sh) -> Self
    where
        A: Clone + Zero,
        S: DataOwned,
        Sh: ShapeBuilder<Dim = D>,
    {
        new!(ArrayBase::zeros(shape))
    }

    pub fn zeros_like(&self) -> Self
    where
        A: Clone + Zero,
        S: DataOwned,
    {
        Self::zeros(self.dim())
    }
}
