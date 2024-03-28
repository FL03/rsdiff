/*
    Appellation: tensor <example>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
#![cfg(feature = "tensor")]

extern crate acme;

use acme::prelude::{BoxResult, Tensor};

fn main() -> BoxResult {
    let shape = (2, 3);
    let tensor: Tensor<f64> = Tensor::linspace(1.0, 7.0, 6).reshape(shape)?;
    let b = tensor.t();
    println!("{:?}", &tensor[&[1, 1]]);
    println!("{:?}", &b[&[1, 1]]);
    Ok(())
}
