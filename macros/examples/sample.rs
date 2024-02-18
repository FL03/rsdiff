/*
    Appellation: simple <example>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
extern crate acme_macros as macros;

use macros::show_streams;

fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    foo();
    Ok(())
}

#[show_streams({ delimeters })]
fn foo() {}
