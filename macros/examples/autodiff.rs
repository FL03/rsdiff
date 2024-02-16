/*
    Appellation: autodiff <example>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
extern crate acme_macros as macros;

use macros::*;

fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    sample_partial();

    Ok(())
}

fn sample_partial() {
    let x = 1.0;
    let y = 2.0;
    let z = partial!(y: y * (x + y));
    println!("Partial Derivative: {:?}", z);
}
