/*
    Appellation: simple <example>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
extern crate acme_macros as macros;

use macros::*;

fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    foo();
    let x = 1.0;
    let y = 2.0;
    let z = partial!(y: x + y;);
    println!("Partial Derivative: {:?}", z);
    Ok(())
}

#[show_streams({ delimeters })]
fn foo() {}
