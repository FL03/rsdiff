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
    let a = Tensor::<f64>::ones(shape);
    let b = Tensor::<f64>::ones(shape);
    let c = &a + &b;
    let grad = c.grad();

    assert_eq!(
        grad[&a.id()],
        Tensor::ones(shape),
        "{:?} != {:?}",
        grad[&a.id()].to_vec(),
        vec![1_f64; 4]
    );
    assert_eq!(grad[&b.id()], Tensor::ones(shape));

    let a = Tensor::<f64>::ones(shape);
    let b = Tensor::<f64>::fill(shape, 2_f64);
    let c = &a * &b;

    let grad = c.grad();

    assert_eq!(grad[&a.id()], Tensor::<f64>::fill(shape, 2_f64));
    assert_eq!(grad[&b.id()], Tensor::ones(shape));
}

#[test]
#[ignore = "Needs to be fixed"]
fn test_add_mul() {
    let shape = (2, 2);
    let a = Tensor::<f64>::ones(shape);
    let b = Tensor::<f64>::ones(shape);
    let c = &a + &b;
    let d = &a * &c;
    let grad = d.grad();

    assert_eq!(grad[&a.id()], Tensor::fill(shape, 3_f64));
    assert_eq!(grad[&b.id()], Tensor::ones(shape));
}
