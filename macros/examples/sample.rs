/*
    Appellation: simple <example>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
extern crate acme_macros as macros;

use macros::show_streams;

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
    (eval $f:expr, ($($xs:ident),*)) => {
        $f($($xs),*)
    }

}

fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    foo();
    Ok(())
}

#[show_streams({ delimeters })]
fn foo() {}