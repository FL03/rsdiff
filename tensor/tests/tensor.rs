/*
    Appellation: tensor <test>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
#![cfg(test)]
extern crate acme_tensor as acme;

use acme::prelude::{IntoShape, Tensor};
use num::One;

#[test]
fn test_tensor() {
    let shape = (2, 2);
    let a = Tensor::<f64>::ones(shape);
    let b = a.zeros_like();

    assert_ne!(a.id(), b.id());
    assert_eq!(a.shape(), b.shape());
    assert_eq!(a.size(), b.size());
    assert_eq!(a.stride(), b.stride());

    let tensor = Tensor::<f64>::one();
    assert!(tensor.is_scalar());
}

#[test]
fn test_first_and_last() {
    let shape = (3, 3);
    let tensor = Tensor::linspace(0f64, 9f64, 9).reshape(shape).unwrap();

    assert_eq!(tensor.first(), Some(&0f64));
    assert_eq!(tensor.last(), Some(&8f64));

    let shape = (3, 3, 1);
    let tensor = Tensor::linspace(0f64, 9f64, 9).reshape(shape).unwrap();

    assert_eq!(tensor.first(), Some(&0f64));
    assert_eq!(tensor.last(), Some(&8f64));
}

#[test]
fn test_index() {
    let shape = (2, 3).into_shape();
    let n = shape.size();
    let a = Tensor::<f64>::linspace(0f64, n as f64, n)
        .reshape(shape)
        .unwrap();

    assert_eq!(a[[0, 0]], 0f64);
    assert_eq!(a[[0, 1]], 1f64);
    assert_eq!(a[[1, 2]], 5f64);
}

#[test]
fn test_higher_dim() {
    let shape = (2, 2, 2, 2);
    let a = Tensor::<f64>::ones(shape);
    let b = a.zeros_like();

    assert_ne!(a.id(), b.id());
    assert_eq!(a.shape(), b.shape());
    assert_eq!(a.size(), b.size());
    assert_eq!(a.stride(), b.stride());
    assert_eq!(a.stride().len(), 4);
}
