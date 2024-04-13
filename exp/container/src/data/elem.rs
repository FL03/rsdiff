/*
    Appellation: elem <mod>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
//! # Elements
//!
//!
use acme::prelude::DType;

pub trait Element {
    type Elem;

    fn dtype(&self) -> DType;
}
