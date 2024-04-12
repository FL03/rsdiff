/*
    Appellation: fields <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
//! # Fields
//!
//!

pub trait Field {
    type Elem: ?Sized;

    fn rank(&self) -> usize;
}

pub trait Scalar {
    type Complex: Scalar;
    type Real: Scalar;
}

#[cfg(test)]
mod tests {}
