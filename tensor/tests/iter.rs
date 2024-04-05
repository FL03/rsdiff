/*
    Appellation: iter <test>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
#![cfg(test)]
extern crate acme_tensor as acme;

use acme::prelude::{IntoShape, Tensor};

#[test]
fn test_sum() {
    let shape = (2, 2).into_shape();
    let a = Tensor::fill(shape, 1f64);
    assert_eq!(a.sum(), 4.0);
}

#[test]
fn test_product() {
    let shape = (2, 2).into_shape();
    let a = Tensor::fill(shape, 2f64);
    assert_eq!(a.product(), 16.0);
}
