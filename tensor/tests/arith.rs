/*
    Appellation: arith <test>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
#![cfg(test)]
extern crate acme_tensor as acme;

use acme::TensorBase;
// use acme::prelude::Matmul;

#[test]
fn test_add() {
    let shape = (2, 2);
    let a = TensorBase::<f64>::ones(shape);
    let b = TensorBase::<f64>::ones(shape);
    let c = a + &b;

    assert_eq!(c, TensorBase::fill(shape, 2_f64));

    let a = TensorBase::<f64>::ones(shape);
    let b = a + 1_f64;
    assert_eq!(b, TensorBase::fill(shape, 2_f64));
}

#[test]
fn test_div() {
    let shape = (2, 2);
    let a = TensorBase::<f64>::ones(shape);
    let b = TensorBase::<f64>::ones(shape) * 2.0;
    let c = a / b;

    assert_eq!(c, TensorBase::<f64>::fill(shape, 0.5));
}

#[test]
fn test_mul() {
    let shape = (2, 2);
    let a = TensorBase::<f64>::ones(shape);
    let b = TensorBase::<f64>::ones(shape);
    let c = a * b;

    assert_eq!(c, TensorBase::<f64>::ones(shape));
}

#[test]
fn test_sub() {
    let shape = (2, 2);
    let a = TensorBase::<f64>::ones(shape);
    let b = TensorBase::<f64>::ones(shape);
    let c = a - &b;

    assert_eq!(c, TensorBase::<f64>::zeros(shape));
}

#[test]
fn test_matmul() {
    let a = TensorBase::<f64>::fill((3, 2), 2_f64);
    let b = TensorBase::<f64>::ones((2, 3));
    let c = a.matmul(&b);

    assert_eq!(c, TensorBase::<f64>::fill((3, 3), 4.0));
}

#[test]
fn test_trig() {
    let a = TensorBase::<f64>::ones((2, 2));
    let b = a.clone().sin();
    let c = a.cos();

    assert_eq!(b[&[0, 0]], 1_f64.sin());
    assert_eq!(c[&[0, 0]], 1_f64.cos());
}
