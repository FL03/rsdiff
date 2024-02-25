/*
    Appellation: params <example>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
extern crate acme_derive as acme;

use acme::Params;

fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let _params = LinearParams { weight: 1.0 };
    let wk = LinearParamsKey::Weight;
    println!("{:?}", &wk);
    // let _key = wk.key();
    Ok(())
}

#[derive(Params)]
pub struct LinearParams<T> {
    #[param]
    pub weight: T,
}
