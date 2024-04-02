/*
    Appellation: linalg <test>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
#![cfg(test)]
extern crate acme_tensor as acme;

use acme::prelude::{Matmul, Shape, Tensor};

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

#[ignore = "not implemented"]
#[test]
fn test_inverse() {
    let shape = Shape::from((2, 2));
    let arr: Vec<f64> = vec![1.0, 4.0, 3.0, 2.0];
    let tensor = Tensor::from_shape_vec(shape.clone(), arr);
    let inv_arr = vec![-0.2, 0.4, 0.3, -0.1];
    let exp = Tensor::from_shape_vec(shape.clone(), inv_arr);

    let inverse = tensor.inv().unwrap();

    for i in 0..shape.nrows() {
        for j in 0..shape.ncols() {
            assert_diff!(inverse[[i, j]], exp[[i, j]]);
        }
    }
}

#[test]
fn test_matmul() {
    let a = Tensor::<f64>::fill((3, 2), 2_f64);
    let b = Tensor::<f64>::ones((2, 3));
    let c = a.matmul(&b);

    assert_eq!(c, Tensor::<f64>::fill((3, 3), 4.0));
}
