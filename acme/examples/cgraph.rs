/*
    Appellation: simple <example>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
extern crate acme;

use acme::graph::scg::Scg;
use acme::prelude::Result;

fn main() -> Result<()> {
    let mut scg = Scg::new();
    let x = scg.variable(1.0);
    let y = scg.variable(2.0);

    let z = scg.add(x, y)?;
    let w = scg.mul(z, y)?;

    let eval = scg.get_value(w).unwrap();
    println!("{:?}", *eval);

    let grad = scg.backward();
    println!("{:?}", grad);

    Ok(())
}
