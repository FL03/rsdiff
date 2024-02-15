/*
    Appellation: params <example>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
extern crate acme_derive as acme;

use acme::Params;

fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>>{
    let params = LinearParams { weight: 1.0 };
    Ok(())
}


#[derive(Params)]
pub struct LinearParams {
    #[param]
    pub weight: f64,
}

