/*
    Appellation: autodiff <example>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
#![feature(fn_traits)]
extern crate acme;

use acme::{autodiff, show_item};
use acme::prelude::sigmoid;

macro_rules! eval {
    ($var:ident: $ex:expr) => {
        println!("Eval: {:?}", $ex);
        println!("Gradient: {:?}", autodiff!($var: $ex));
    }
}

fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let x: f64 = 2.0;

    eval!(x: x.tan());

    eval!(x: x.sin());

    eval!(x: x.cos().sin());
    // show_item!(sigmoid::<f64>);
    unsafe {
        println!("{:?}", sigmoid::<f64>.call((2_f64,)));
    }
    

    Ok(())
}
