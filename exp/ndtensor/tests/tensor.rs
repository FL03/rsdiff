/*
    Appellation: tensor <test>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
#![cfg(test)]

extern crate ndtensor;

use ndarray::array;
use ndtensor::prelude::Tensor;

#[test]
fn test_tensor() {
    let tensor = Tensor::new(array![[0f64, 1f64], [2f64, 3f64]]);

    assert!(tensor.op().is_none());

}