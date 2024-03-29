/*
    Appellation: index <mod>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
//! # Index
//!
//!
pub use self::{slice::*, strides::*};

pub(crate) mod slice;
pub(crate) mod strides;

use crate::tensor::TensorBase;

pub enum IndexItem<T> {
    Scalar(T),
    Strides(TensorBase<T>),
}

#[cfg(test)]
mod tests {
    use super::Strides;
    use crate::prelude::Shape;
    use crate::tensor::TensorBase;

    #[test]
    fn test() {
        let shape = Shape::from_iter([2, 2]);
        let n = shape.size();
        let tensor = TensorBase::linspace(0f64, n as f64, n)
            .reshape(shape)
            .unwrap();
        let indexer = Strides::from(tensor.layout());
        for (i, idx) in indexer.enumerate() {
            let elem = *tensor.get_by_index(idx).unwrap();
            println!("{:?}", &elem);

            assert_eq!(i as f64, elem);
        }
    }
}
