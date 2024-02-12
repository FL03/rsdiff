/*
    Appellation: simple <example>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
extern crate acme_core as acme;

use acme::prelude::BoxResult;
use num::Float;

macro_rules! autodiff {
    (eval $f:expr) => {
        $f()
    };
    (grad $df:expr) => {
        $df()
    }
}

macro_rules! func {
    ($f:expr) => {
        $f
    };
    (eval $f:expr, $($xs:ident),*) => {
        $f($($xs),*)
    }

}

fn main() -> BoxResult {
    let (a, b, c) = (1.0, 2.0, 3.0);
    let f = func!(|x, y, z| z * (x + y));
    println!("{:?}", f(a, b, c));
    Ok(())
}

