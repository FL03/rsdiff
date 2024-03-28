/*
    Appellation: reshape <test>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
#![cfg(test)]
extern crate acme_tensor as acme;

use acme::prelude::Tensor;

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

    assert_ne!(&a.shape(), &b.shape());
    assert_eq!(&a.size(), &b.size());
}

#[test]
fn test_transpose() {
    let shape = (2, 3);
    let a = Tensor::<f64>::linspace(0.0, 6.0, 6).with_shape(shape);
    let at = a.t();
    println!("Transposed Shape: {:?}", &at.shape());

    let exp = Tensor::from_vec(false, None, (3, 2), vec![0.0, 3.0, 1.0, 4.0, 2.0, 5.0]);
    assert_ne!(&a, &at);
    assert_eq!(at.shape(), (3, 2).into());
    for i in 0..shape.0 {
        for j in 0..shape.1 {
            assert_eq!(a[&[i, j]], exp[&[j, i]]);
        }
    }
}
