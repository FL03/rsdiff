/*
    Appellation: operators <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
use crate::exp::Config;

pub struct Operator<C: Config> {
    pub inputs: Vec<C::DType>,
    pub operation: C::Eval,
}

impl<C> Operator<C> where C: Config {
    pub fn new() -> Self {
        Self { inputs: Vec::new(), operation: C::Eval::default() }
    }
}
