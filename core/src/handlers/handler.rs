/*
    Appellation: handler <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... summary ...
*/
use serde::{Deserialize, Serialize};

pub type TransitionFunction<S, T> = dyn Fn(S) -> T;

pub trait Transition<S> {
    type Output;

    fn data(&self) -> S;
    fn transition(&self, dirac: &TransitionFunction<S, Self::Output>) -> Self::Output {
        dirac(self.data())
    }
}

#[derive(Clone, Debug, Default, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct Handler;
