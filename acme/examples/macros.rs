/*
    Appellation: autodiff <example>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
extern crate acme;

use acme::{autodiff, operator};
use num::Float;

fn main() -> acme::prelude::BoxResult {
    let x = 5f64;
    println!("{}", sigmoid(x));
    // println!("{}", sigmoid_lex());
    let dx = sigmoid_prime(x);
    // assert_eq!(dx, autodiff!(x: SIGMOID_LEXICAL));
    println!("{}", dx);

    Ok(())
}

#[operator()]
pub fn sigmoid<T>(x: T) -> T
where
    T: Float,
{
    x.exp() / (T::one() + x.exp())
}

pub fn sigmoid_prime<T>(x: T) -> T
where
    T: Float,
{
    sigmoid(x) * (T::one() - sigmoid(x))
}
