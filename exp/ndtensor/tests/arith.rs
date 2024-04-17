/*
    Appellation: tensor <test>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
#![cfg(test)]

extern crate ndtensor;

use ndarray::Ix2;
use ndtensor::prelude::Tensor;

#[test]
fn test_addition() {
    let shape = (3, 3);

    let a = Tensor::<f64, Ix2>::linshape(shape.clone()).unwrap();
    let b = Tensor::<f64, Ix2>::ones(shape);

    let res = a + b;

    assert_eq!(res.data().sum(), 45f64);
}
