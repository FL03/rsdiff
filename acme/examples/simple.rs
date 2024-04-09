/*
    Appellation: simple <example>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
extern crate acme;

use acme::prelude::{nested, BoxResult};

fn main() -> BoxResult {
    nested!(
        for i in 0..3,
        for j in 0..3,
        for k in 0..3 => {
        println!("({}, {}, {})", i, j, k)
    });
    Ok(())
}
