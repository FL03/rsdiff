/*
    Appellation: tensor <test>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
#![cfg(test)]

extern crate ndtensor;

use ndarray::array;
use ndtensor::prelude::{hash_dim, Tensor};

#[test]
fn test_tensor() {
    let tensor = Tensor::ndtensor(array![[0f64, 1f64], [2f64, 3f64]]);

    assert!(tensor.op().is_none());
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
