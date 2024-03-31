/*
    Appellation: tensor <bench>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
#![feature(test)]
extern crate acme;
extern crate test;

use acme::prelude::{IntoShape, Tensor};
use test::Bencher;



#[bench]
fn tensor_iter(b: &mut Bencher) {
    let shape = (20, 20, 20).into_shape();
    let n = shape.size();
    let tensor = Tensor::linspace(0f64, n as f64, n);
    b.iter(|| tensor.strided().take(n))
}
