/*
    Appellation: backward <test>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
#![cfg(test)]
extern crate acme_tensor as acme;

use acme::prelude::{IntoShape, Scalar, Tensor};
use core::ops::Neg;

fn shapespace<T>(shape: impl IntoShape) -> Tensor<T> where T: PartialOrd + Scalar {
    let shape = shape.into_shape();
    Tensor::<T>::linspace(T::zero(), T::from(shape.size()).unwrap(), shape.size()).reshape(shape).unwrap()
}

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
#[ignore = "Fix: test throws an error"]
fn test_sigmoid() {
    let shape = (2, 2).into_shape();
    let a = shapespace::<f64>(shape.clone()).variable();
    let b = shapespace::<f64>(shape);

    println!("({}({}), {})", a.kind(), a.id(), b.id());
    let res = a.sigmoid();
    let grad = a.sigmoid().grad().unwrap();
    let exp = (&b).neg().exp() / ((&b).neg().exp() + &b.ones_like()).powi(2);
    println!("{:?}", &grad);
    assert_eq!(
        grad[&a.id()],
        exp.detach(),
        "Gradient of sigmoid is incorrect"
    );
}
