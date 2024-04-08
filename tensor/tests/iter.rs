/*
    Appellation: iter <test>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
#![cfg(test)]
extern crate acme_tensor as acme;

use acme::create::Linspace;
use acme::prelude::{IntoShape, Layout, Shape, Tensor};
use num::traits::{FromPrimitive, Num};

fn linvec<T>(n: usize) -> (Vec<T>, usize)
where
    T: Copy + Default + FromPrimitive + Num + PartialOrd,
{
    let space = Vec::linspace(T::zero(), T::from_usize(n).unwrap(), n);
    (space, n)
}

#[test]
fn test_layout_iter() {
    let shape = (2, 2).into_shape();
    let layout = Layout::contiguous(shape);
    let exp = [vec![0usize, 0], vec![0, 1], vec![1, 0], vec![1, 1]];
    for (pos, exp) in layout.iter().zip(exp.iter()) {
        assert_eq!(pos.position(), *exp);
    }
    for (pos, exp) in layout.iter().rev().zip(exp.iter().rev()) {
        assert_eq!(pos.position(), *exp);
    }
}

#[test]
fn test_iter() {
    let shape = Shape::from_iter([2, 2, 2, 2]);
    let (exp, n) = linvec::<f64>(shape.size());
    let tensor = Tensor::linspace(0f64, n as f64, n)
        .reshape(shape.clone())
        .unwrap();
    assert_eq!(&tensor, &exp);

    let mut tensor = Tensor::zeros(shape);
    for (elem, val) in tensor.iter_mut().zip(exp.iter()) {
        *elem = *val;
    }
    assert_eq!(&tensor, &exp);
}
    #[test]
fn test_iter_scalar() {
    let exp = 10f64;
    let tensor = Tensor::from_scalar(exp);
    let mut iter = tensor.iter();
    assert_eq!(iter.next(), Some(&exp));
    assert_eq!(iter.next(), None);
}

#[test]
fn test_iter_mut_rev() {
    let shape = Shape::from_iter([2, 2, 2, 2]);
    let n = shape.size();
    let exp = Vec::linspace(0f64, n as f64, n);
    let rev = exp.iter().rev().copied().collect::<Vec<f64>>();
    let mut tensor = Tensor::zeros(shape);
    for (elem, val) in tensor.iter_mut().rev().zip(exp.iter()) {
        *elem = *val;
    }
    assert_eq!(&tensor, &rev);
}

#[test]
fn test_iter_rev() {
    let shape = Shape::from_iter([2, 2]);
    let n = shape.size();
    let exp = Vec::linspace(0f64, n as f64, n);
    let tensor = Tensor::linspace(0f64, n as f64, n).reshape(shape).unwrap();

    for (i, j) in tensor.iter().rev().zip(exp.iter().rev()) {
        assert_eq!(i, j);
    }
}

#[test]
fn test_sum() {
    let shape = (2, 2).into_shape();
    let a = Tensor::fill(shape, 1f64);
    assert_eq!(a.sum(), 4.0);
}

#[test]
fn test_product() {
    let shape = (2, 2).into_shape();
    let a = Tensor::fill(shape, 2f64);
    assert_eq!(a.product(), 16.0);
}
