/*
    Appellation: indexed <mod>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
use crate::prelude::{Layout, Shape, Stride};
use crate::tensor::TensorBase;

pub struct IndexedIter<'a, T: 'a> {
    
    next: Option<usize>,
    scope: Option<&'a T>,
    tensor: &'a TensorBase<T>,
}
