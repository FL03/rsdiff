/*
    Appellation: backward <test>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
#![cfg(test)]
extern crate acme_tensor as acme;

use acme::prelude::{IntoShape, Tensor};
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
    let b = Tensor::<f64>::fill(shape, 2f64).variable();

    let res = &b * (&a + &b);

    let grad = res.grad().unwrap();

    assert_eq!(grad[&a.id()], Tensor::fill(shape, 2f64));
    assert_eq!(grad[&b.id()], Tensor::fill(shape, 5f64));
}

#[test]
fn test_complex_expr() {
    let shape = (2, 2);

    let a = Tensor::<f64>::ones(shape).variable();
    let b = Tensor::fill(shape, 2f64).variable();
    let c = Tensor::fill(shape, 3f64).variable();
    let res = (&a + &b) * c.sin() + &b;

    let grad = res.grad().unwrap();

    assert_eq!(grad[&a.id()], c.sin());
    assert_eq!(grad[&b.id()], c.sin() + 1f64);
    assert_eq!(grad[&c.id()], (&a + &b) * c.cos());
}

#[test]
// #[ignore = "This test is not yet implemented"]
fn test_sigmoid() {
    let shape = (2, 2).into_shape();
    let n = shape.size();
    let a = Tensor::<f64>::linspace(0f64, n as f64, n).reshape(shape.clone()).unwrap().variable();
    let b = Tensor::<f64>::linspace(0f64, n as f64, n).reshape(shape.clone()).unwrap();
    let res = ((-&a).exp() + 1f64).recip();

    let grad = res.grad().unwrap();

    let exp = (&b).neg().exp() / ((&b).neg().exp() + 1f64).powi(2);


    assert_eq!(grad[&a.id()].detach(), exp.detach(), "Gradient of sigmoid is incorrect");
}

