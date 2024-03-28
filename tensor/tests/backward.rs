/*
    Appellation: backward <test>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
#![cfg(test)]
extern crate acme_tensor as acme;

use acme::prelude::Tensor;
use core::ops::Neg;

#[test]
fn test_backward() {
    let shape = (2, 2);
    let a = Tensor::<f64>::ones(shape).variable();
    let grad = a.grad().unwrap();

    assert_eq!(grad[&a.id()], Tensor::ones(shape),);
}

#[test]
fn test_addition() {
    let shape = (2, 2);
    let a = Tensor::<f64>::ones(shape).variable();
    let b = Tensor::<f64>::ones(shape).variable();
    let c = &a + &b;
    let grad = c.grad().unwrap();

    assert_eq!(grad[&a.id()], Tensor::ones(shape));
    assert_eq!(grad[&b.id()], Tensor::ones(shape));
}

#[test]
fn test_addition_2() {
    let shape = (2, 2);
    let a = Tensor::<f64>::ones(shape).variable();
    let b = Tensor::<f64>::ones(shape).variable();
    let c = Tensor::<f64>::ones(shape).variable();
    let d = &a + &b + &c;

    assert_eq!(&d, &Tensor::fill(shape, 3_f64));

    let grad = d.grad().unwrap();

    for i in [a.id(), b.id(), c.id()].iter() {
        assert_eq!(grad[i], Tensor::ones(shape));
    }
}

#[test]
fn test_division() {
    let shape = (2, 2);

    let a = Tensor::<f64>::ones(shape).variable();
    let b = Tensor::<f64>::fill(shape, 2_f64).variable();
    let c = &a / &b;

    let grad = c.grad().unwrap();

    assert_eq!(grad[&a.id()], Tensor::fill(shape, 0.5));
    assert_eq!(grad[&b.id()], Tensor::fill(shape, -0.25));
}

#[test]
fn test_multiplication() {
    let shape = (2, 2);

    let a = Tensor::<f64>::ones(shape).variable();
    let b = Tensor::<f64>::fill(shape, 2_f64).variable();
    let c = &a * &b;

    let grad = c.grad().unwrap();

    assert_eq!(grad[&a.id()], Tensor::fill(shape, 2_f64));
    assert_eq!(grad[&b.id()], Tensor::ones(shape));
}

#[test]
fn test_subtraction() {
    let shape = (2, 2);

    let a = Tensor::<f64>::ones(shape).variable();
    let b = Tensor::<f64>::fill(shape, 2_f64).variable();
    let c = &a - &b;

    let grad = c.grad().unwrap();

    assert_eq!(grad[&a.id()], Tensor::ones(shape));
    assert_eq!(grad[&b.id()], Tensor::ones(shape).neg());
}

#[test]
fn test_mixed() {
    let shape = (2, 2);

    let a = Tensor::<f64>::ones(shape).variable();
    let b = Tensor::<f64>::fill(shape, 2_f64).variable();

    let res = &b * (&a + &b);

    let grad = res.grad().unwrap();

    assert_eq!(grad[&a.id()], Tensor::fill(shape, 2_f64));
    assert_eq!(grad[&b.id()], Tensor::fill(shape, 5_f64));
}
