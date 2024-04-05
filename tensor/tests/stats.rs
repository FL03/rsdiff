/*
    Appellation: stats <test>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
#![cfg(test)]
extern crate acme_tensor as acme;

use acme::prelude::{Shape, Tensor};

macro_rules! adiff {
    ($a:expr, $b:expr) => {
        ($a - $b).abs()
    };
}
macro_rules! assert_diff {
    ($a:expr, $b:expr, $tol:expr) => {
        let diff = adiff!($a, $b);
        assert!(
            diff < $tol,
            "the difference ({}) between {} and {} exceeds the allowed tolerance",
            diff,
            $a,
            $b
        );
    };
    ($a:expr, $b:expr) => {
        assert_diff!($a, $b, 1e-10);
    };
}

#[test]
fn test_std() {
    let shape = Shape::from((2, 2));
    let tensor = Tensor::linspace(0f64, shape.size() as f64, shape.size())
        .reshape(shape)
        .unwrap();
    let exp = 1.118033988749895;
    assert_diff!(tensor.std(), exp);
    assert_diff!(tensor.variance(), exp.powi(2));
}
