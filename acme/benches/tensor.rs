/*
    Appellation: tensor <bench>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
#![feature(test)]
extern crate acme;
extern crate test;

use acme::prelude::{IntoShape, Shape, Tensor};
use lazy_static::lazy_static;
use test::Bencher;

lazy_static! {
    static ref SHAPE_3D: Shape = SHAPE_3D_PATTERN.into_shape();
}

const SHAPE_3D_PATTERN: (usize, usize, usize) = (100, 10, 1);

#[bench]
fn bench_iter(b: &mut Bencher) {
    let shape = SHAPE_3D.clone();
    let n = shape.size();
    let tensor = Tensor::linspace(0f64, n as f64, n);
    b.iter(|| tensor.strided().take(n))
}

#[bench]
fn bench_iter_rev(b: &mut Bencher) {
    let shape = SHAPE_3D.clone();
    let n = shape.size();
    let tensor = Tensor::linspace(0f64, n as f64, n);
    b.iter(|| tensor.strided().rev().take(n))
}
