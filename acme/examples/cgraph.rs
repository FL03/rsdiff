/*
    Appellation: simple <example>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
extern crate acme;

use acme::prelude::{Graph, Result};

fn main() -> Result<()> {
    let mut dcg = Graph::new();
    let x = dcg.variable(1.0);
    let y = dcg.variable(2.0);

    let z = dcg.add(x, y)?;
    let w = dcg.mul(z, y)?;

    let eval = dcg.get_value(w).unwrap();
    println!("{:?}", *eval);

    let grad = dcg.backward();
    println!("{:?}", grad);

    Ok(())
}
