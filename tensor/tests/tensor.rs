/*
    Appellation: tensor <test>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
#![cfg(test)]
extern crate acme_tensor as acme;

use acme::prelude::Tensor;

#[test]
fn test_tensor() {
    let shape = (2, 2);
    let a = Tensor::<f64>::ones(shape);
    let b = a.zeros_like();

    assert_ne!(a.id(), b.id());
    assert_eq!(a.shape(), b.shape());
    assert_eq!(a.size(), b.size());
    assert_eq!(a.stride(), b.stride());
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
