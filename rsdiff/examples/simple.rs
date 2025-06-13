/*
    Appellation: simple <example>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
extern crate rsdiff;

use rsdiff::prelude::{BoxResult, nested};

fn main() -> BoxResult {
    nested!(
        i in 0..3 => j in 0..3 => k in 0..3 => {
        println!("({}, {}, {})", i, j, k)
    });
    Ok(())
}
