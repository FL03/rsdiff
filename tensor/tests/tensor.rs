/*
    Appellation: tensor <test>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
#![cfg(test)]
extern crate acme_tensor as tensor;

use tensor::TensorBase;

#[test]
fn test_tensor() {
    let shape = (2, 2);
    let a = TensorBase::<f64>::ones(shape);
    let b = TensorBase::<f64>::zeros(shape);

    assert_ne!(&a, &b);
}
