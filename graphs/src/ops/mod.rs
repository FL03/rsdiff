/*
    Appellation: ops <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
//! # Operations
//!
//!
pub use self::kinds::*;

pub(crate) mod kinds;

pub trait Operator {
    fn boxed(self) -> Box<dyn Operator>
    where
        Self: Sized + 'static,
    {
        Box::new(self)
    }
    fn name(&self) -> String;
}

impl Operator for Box<dyn Operator> {
    fn name(&self) -> String {
        self.as_ref().name()
    }
}
