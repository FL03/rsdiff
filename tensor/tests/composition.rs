/*
    Appellation: tensor <test>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
#![cfg(test)]
extern crate acme_tensor as acme;

use acme::prelude::{Shape, Tensor};

#[test]
fn test_tensor() {
    let shape = (2, 2);
    let a = Tensor::<f64>::ones(shape);
    let b = Tensor::zeros(shape);

    assert_ne!(&a, &b);
}

#[test]
fn test_reshape() {
    let shape = (2, 2);
    let a = Tensor::<f64>::ones(shape);
    let b = a.clone().reshape((4,)).unwrap();

    assert_ne!(&a.shape(), &b.shape());
    assert_eq!(&a.elements(), &b.elements());
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
