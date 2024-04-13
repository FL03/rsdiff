/*
    Appellation: specs <library>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
use crate::prelude::{ArrayBase, Dimension, IxDyn};
use ndarray::RawData;

pub trait NdTensor<S, D = IxDyn>
where
    D: Dimension,
    S: RawData,
{
    fn container(&self) -> ArrayBase<S, D>;
    fn shape(&self) -> Vec<usize>;

    fn rank(&self) -> usize {
        self.shape().len()
    }
}
