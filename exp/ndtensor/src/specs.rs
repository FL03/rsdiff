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
    fn data(&self) -> ArrayBase<S, D>;

    fn dim(&self) -> D;

    fn rank(&self) -> usize {
        D::NDIM.unwrap_or(self.dim().slice().len())
    }
}
