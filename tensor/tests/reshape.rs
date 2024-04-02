/*
    Appellation: reshape <test>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
#![cfg(test)]
extern crate acme_tensor as acme;

use acme::prelude::{Shape, Tensor};

#[test]
#[ignore = "Not implemented"]
fn test_broadcast() {
    let shape = (4, 1);
    let a = Tensor::<f64>::ones(shape);
    let b = a.clone().broadcast((4, 1, 1));

    assert_ne!(&a.shape(), &b.shape());
    assert_eq!(&a.size(), &b.size());
}

#[test]
fn test_reshape() {
    let shape = (2, 2);
    let a = Tensor::<f64>::ones(shape);
    let b = a.clone().reshape((4,)).unwrap();

    assert_ne!(a.rank(), b.rank());
    assert_ne!(a.shape(), b.shape());
    assert_eq!(a.size(), b.size());
}

#[test]
fn test_transpose() {
    let shape = (2, 3);
    let a = Tensor::<f64>::linspace(0f64, 6f64, 6).with_shape(shape);
    let at = a.t();

    let exp = Tensor::from_shape_vec((3, 2), vec![0.0, 3.0, 1.0, 4.0, 2.0, 5.0]);
    assert_ne!(&a, &at);
    assert_eq!(at.shape(), &Shape::new(vec![3, 2]));
    for i in 0..shape.0 {
        for j in 0..shape.1 {
            assert_eq!(a[&[i, j]], exp[&[j, i]]);
        }
    }
}
