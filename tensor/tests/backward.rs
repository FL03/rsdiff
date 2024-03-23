/*
    Appellation: backward <test>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
#![cfg(test)]
extern crate acme_tensor as acme;

use acme::prelude::Tensor;

#[test]
fn test_backward() {
    let shape = (2, 2);
    let a = Tensor::<f64>::ones(shape).variable();
    let b = Tensor::<f64>::ones(shape).variable();
    let c = &a + &b;
    let grad = c.grad();

    assert_eq!(grad[&a.id()], Tensor::ones(shape),);
    assert_eq!(grad[&b.id()], Tensor::ones(shape));

    let a = Tensor::<f64>::ones(shape).variable();
    let b = Tensor::<f64>::fill(shape, 2_f64).variable();
    let c = &a * &b;

    let grad = c.grad();

    assert_eq!(grad[&a.id()], Tensor::fill(shape, 2_f64));
    assert_eq!(grad[&b.id()], Tensor::ones(shape));
}

#[test]
#[ignore = "Needs to be fixed"]
fn test_add_chain() {
    let shape = (2, 2);

    let a = Tensor::<f64>::ones(shape).variable();
    let b = Tensor::<f64>::fill(shape, 2_f64).variable();
    let c = &a + &b;
    let d = &c + &a;

    let grad = d.grad();
    // println!("Gradient:\n\n{:?}\n\n", &grad);

    assert_eq!(grad[&a.id()], Tensor::fill(shape, 2_f64));
    assert_eq!(grad[&b.id()], Tensor::ones(shape));
}

#[test]
#[ignore = "Needs to be fixed"]
fn test_add_mul() {
    let shape = (2, 2);
    let a = Tensor::<f64>::ones(shape).variable();
    let b = Tensor::<f64>::ones(shape).variable();
    println!("A({}), B({})", a.id(), b.id());
    let c = &a + &b;
    let d = &a * &c;
    let grad = d.grad();

    assert_eq!(grad[&a.id()], Tensor::fill(shape, 3_f64));
    assert_eq!(grad[&b.id()], Tensor::ones(shape));
}
