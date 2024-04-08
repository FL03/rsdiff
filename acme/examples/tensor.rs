/*
    Appellation: tensor <example>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
#![cfg(feature = "tensor")]

extern crate acme;

use acme::prelude::{BoxResult, IntoShape, Linspace, Matmul, Tensor};

fn main() -> BoxResult {
    let shape = (3, 3);

    tensor_iter_mut(shape)?;
    Ok(())
}

pub fn example_matmul() -> BoxResult<Tensor<f64>> {
    let shape = (2, 3);
    let tensor: Tensor<f64> = Tensor::linspace(1.0, 7.0, 6).reshape(shape)?;
    let b = tensor.t();
    let c = tensor.matmul(&b);
    println!("{:?}", &c);
    Ok(c)
}

pub fn tensor_iter_mut(shape: impl IntoShape) -> BoxResult<Tensor<f64>> {
    let shape = shape.into_shape();
    let n = shape.size();
    let exp = Vec::linspace(0f64, n as f64, n);
    let mut tensor = Tensor::zeros(shape);
    assert_ne!(&tensor, &exp);
    for (elem, val) in tensor.iter_mut().zip(exp.iter()) {
        *elem = *val;
    }
    assert_eq!(&tensor, &exp);
    println!("{:?}", Vec::from_iter(&mut tensor.iter().rev()));
    Ok(tensor)
}

pub fn tensor_iter_mut_rev(shape: impl IntoShape) -> BoxResult<Tensor<f64>> {
    let shape = shape.into_shape();
    let n = shape.size();
    let exp = Vec::linspace(0f64, n as f64, n);
    let mut tensor = Tensor::zeros(shape.clone());
    assert_ne!(&tensor, &exp);
    for (elem, val) in tensor.iter_mut().rev().zip(exp.iter()) {
        *elem = *val;
    }
    // assert_eq!(&tensor, &exp);
    let sample = Tensor::linspace(0f64, n as f64, n).reshape(shape)?;
    println!("*** Reversed ***");
    for i in sample.clone().iter().copied().rev() {
        println!("{:?}", i);
    }

    Ok(tensor)
}
