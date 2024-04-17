/*
    Appellation: tensor <test>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
#![cfg(test)]

extern crate ndtensor;

use approx::AbsDiffEq;
use ndarray::array;
use ndtensor::prelude::{hash_dim, TensorBase};

#[test]
fn test_tensor() {
    let tensor = TensorBase::ndtensor(array![[0f64, 1f64], [2f64, 3f64]]);

    assert!(tensor.op().is_none());
}

#[test]
fn test_index() {
    let tensor = TensorBase::ndtensor(array![[0f64, 1f64], [2f64, 3f64]]);

    assert_eq!(tensor[[0, 0]], 0f64);
}

#[test]
fn test_tensor_ops() {
    let tensor = TensorBase::ndtensor(array![[0f64, 1f64], [2f64, 3f64]]);
    let res = tensor.cos();
    tensor
        .data()
        .mapv(|i| i.cos())
        .abs_diff_eq(&res.data(), 1e-8);
}

#[test]
fn test_dim_hash() {
    // hash: 14739029374222202580
    let s1 = (3, 3);
    let s2 = [3, 3];
    let s3 = vec![3, 3];
    assert_eq!(hash_dim(s1), 1069660947015105383);
    assert_eq!(hash_dim(s1), hash_dim(s2));
    assert_eq!(hash_dim(s1), hash_dim(s3));
}
