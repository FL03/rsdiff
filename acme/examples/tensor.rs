/*
    Appellation: tensor <example>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
#![cfg(feature = "tensor")]

extern crate acme;

use acme::prelude::{Axis, BoxResult, IntoShape, Linspace, Matmul, Tensor};

fn main() -> BoxResult {
    let shape = (3, 3);

    // tensor_iter_mut(shape)?;
    axis_iter(shape, 0)?;
    Ok(())
}

pub fn axis_iter(shape: impl IntoShape, axis: usize) -> BoxResult<Tensor<f64>> {
    let axis = Axis::new(axis);
    let shape = shape.into_shape();
    let n = shape.size();
    let tensor = Tensor::linspace(0f64, n as f64, n).reshape(shape.clone())?;

    let mut res = Vec::new();
    for i in 0..tensor.shape()[axis] {
        let mut tmp = Tensor::zeros(shape.ncols());
        for k in 0..shape.ncols() {
            tmp[[k]] = tensor[[i, k]];
        }
        res.push(tmp);
    }
    for i in res {
        println!("{:?}", &i.to_vec());
    }
    Ok(tensor)
}

#[allow(dead_code)]
pub fn axis_iter_impl(shape: impl IntoShape, axis: usize) -> BoxResult<Tensor<f64>> {
    let axis = Axis::new(axis);
    let shape = shape.into_shape();
    let n = shape.size();
    let tensor = Tensor::linspace(0f64, n as f64, n).reshape(shape.clone())?;

    let ns = tensor.layout().remove_axis(axis);
    let mut res = Vec::new();
    for i in 0..tensor.shape()[axis] {
        for j in ns.shape().iter().copied() {
            let mut tmp = Tensor::zeros(j);
            for k in 0..ns.shape()[j] {
                tmp[[k]] = tensor[[i, k]];
            }
            res.push(tmp);
        }
    }
    for i in res {
        println!("{:?}", &i.to_vec());
    }
    Ok(tensor)
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
