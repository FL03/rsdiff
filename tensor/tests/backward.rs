/*
    Appellation: backward <test>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
#![cfg(test)]
extern crate acme_tensor as acme;

use acme::prelude::{IntoShape, Scalar, Tensor, TensorKind};
use core::ops::Neg;

fn _shapespace<T>(shape: impl IntoShape) -> Tensor<T>
where
    T: PartialOrd + Scalar,
{
    let shape = shape.into_shape();
    Tensor::<T>::linspace(T::zero(), T::from(shape.size()).unwrap(), shape.size())
        .reshape(shape)
        .unwrap()
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
    use acme::prelude::ScalarExt;
    let shape = (2, 2).into_shape();
    let data = (0..shape.size()).map(|x| x as f64).collect::<Vec<_>>();
    let a = Tensor::<f64>::from_shape_vec(shape.clone(), data.clone()).variable();

    let _res = a.sigmoid();
    let grad = _res.grad().unwrap();

    assert_eq!(a.kind(), TensorKind::Variable);

    let exp = Tensor::from_shape_iter(
        shape,
        data.iter().map(|x| x.sigmoid() * (1f64 - x.sigmoid())),
    );
    println!("{:?}", &grad);
    assert_eq!(
        grad[&a.id()],
        exp.detach(),
        "Gradient of sigmoid is incorrect"
    );
}
