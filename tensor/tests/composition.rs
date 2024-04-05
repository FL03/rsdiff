/*
    Appellation: composition <test>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
#![cfg(test)]
extern crate acme_tensor as acme;

use acme::prelude::{Shape, Tensor};

#[test]
fn test_ones_and_zeros() {
    let shape = (2, 2);
    let a = Tensor::<f64>::ones(shape);
    let b = a.zeros_like();

    assert_ne!(&a, &b);
    assert_ne!(a.id(), b.id());
    assert_eq!(a.shape(), b.shape());
    assert_eq!(a.size(), b.size());
    assert_eq!(a.strides(), b.strides());
    assert_eq!(a, Tensor::ones(shape));
    assert_eq!(b, Tensor::zeros(shape));

    use num::traits::{One, Zero};

    assert!(Tensor::<f64>::one().is_scalar());
    assert!(Tensor::<f64>::zero().is_scalar());
}

#[test]
fn test_arange() {
    let exp = Shape::from(10);
    let a = Tensor::arange(0_f64, 10_f64, 1_f64);
    assert_eq!(a.shape(), &exp);

    for i in 0..10 {
        assert_eq!(a[&[i]], i as f64);
    }
}

#[test]
fn test_linstep() {
    let exp = Shape::from(10);
    let a = Tensor::linspace(0_f64, 10_f64, 10);
    assert_eq!(a.shape(), &exp);
    let b = Tensor::arange(0_f64, 10_f64, 1_f64);
    for i in 0..10 {
        assert_eq!(a[&[i]], b[&[i]]);
    }
}

#[test]
fn test_fill() {
    let shape = (2, 2);
    let a = Tensor::fill(shape, 1_f64);
    let b = Tensor::ones(shape);
    assert_eq!(&a, &b);
}
