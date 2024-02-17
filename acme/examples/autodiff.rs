/*
    Appellation: autodiff <example>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
extern crate acme;

use acme::autodiff;

fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let x: f64 = 2.0;

    let eval = x.tan();
    println!("Eval: {:?}", eval);
    let grad = autodiff!(x: x.tan());
    println!("Gradient: {:?}", grad);

    Ok(())
}
