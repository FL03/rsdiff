/*
    Appellation: ops <mod>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
pub use self::backprop::*;

pub(crate) mod backprop;
pub(crate) mod kinds;

#[cfg(test)]
mod tests {}
