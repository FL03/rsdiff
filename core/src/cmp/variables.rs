/*
    Appellation: variables <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
use super::Constant;
use crate::ops::{Evaluate, Gradient};
use num::Num;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct Variable<T> {
    name: String,
    value: Option<T>,
}

impl<T> Variable<T> {
    pub fn new(name: impl ToString) -> Self {
        Self {
            name: name.to_string(),
            value: None,
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn set(&mut self, value: T) {
        self.value = Some(value);
    }
}

impl std::fmt::Display for Variable<f64> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name)
    }
}

impl<T> Evaluate<T> for Variable<T>
where
    T: Clone,
{
    type Output = T;

    fn eval(&self, val: T) -> Self::Output {
        val
    }
}

impl<T> Gradient<Variable<T>> for Variable<T>
where
    T: Num,
{
    type Gradient = Constant<T>;

    fn grad(&self, args: Variable<T>) -> Self::Gradient {
        if self.name() == args.name() {
            // self == args
            return Constant::new(T::one());
        }
        Constant::new(T::zero())
    }
}
