/*
    Appellation: constants <mod>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
use crate::ops::{Evaluate, Gradient};
use num::Zero;
use serde::{Deserialize, Serialize};
use std::ops::{Deref, DerefMut};

#[derive(
    Clone, Copy, Debug, Default, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize,
)]
pub struct Constant<T>(T);

impl<T> Constant<T> {
    pub fn new(value: T) -> Self {
        Self(value)
    }

    pub fn value(&self) -> &T {
        &self.0
    }
}

impl<T> Evaluate<()> for Constant<T>
where
    T: Clone,
{
    type Output = T;

    fn eval(&self, _: ()) -> Self::Output {
        self.0.clone()
    }
}

impl<T> Gradient<T> for Constant<T>
where
    T: Zero,
{
    type Gradient = Constant<T>;

    fn grad(&self, _args: T) -> Self::Gradient {
        Constant::new(T::zero())
    }
}

impl<T> Deref for Constant<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> DerefMut for Constant<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
