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
fn test_arange() {
    let exp = Shape::from(10);
    let a = Tensor::arange(0_f64, 1_f64, 0.1);
    assert_eq!(a.shape(), &exp);
}

#[test]
fn test_fill() {
    let shape = (2, 2);
    let a = Tensor::fill(shape, 1_f64);
    let b = Tensor::ones(shape);
    assert_eq!(&a, &b);
}
