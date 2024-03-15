/*
    Appellation: variables <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
use crate::ops::{Evaluate, Gradient};
use num::{Num, One, Zero};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use std::ops::{Add, Div, Mul, Sub};

#[cfg_attr(feature = "serde", derive(Deserialize, Serialize,))]
#[derive(Clone, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Variable<T> {
    name: String,
    pub(crate) value: Option<T>,
}

impl<T> Variable<T> {
    pub fn new(name: impl ToString) -> Self {
        Self {
            name: name.to_string(),
            value: None,
        }
    }

    pub fn with_name(mut self, name: impl ToString) -> Self {
        self.name = name.to_string();
        self
    }

    pub fn with_value(mut self, value: T) -> Self {
        self.value = Some(value);
        self
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn value(&self) -> Option<&T> {
        self.value.as_ref()
    }

    pub fn set(&mut self, value: T) {
        self.value = Some(value);
    }
}

impl<T> std::fmt::Display for Variable<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name)
    }
}

impl<T> Evaluate for Variable<T>
where
    T: Default,
{
    type Output = T;

    fn eval(self) -> Self::Output {
        self.value.unwrap_or_default()
    }
}

impl<T> Gradient<Variable<T>> for Variable<T>
where
    T: Num,
{
    type Gradient = T;

    fn grad(&self, args: Variable<T>) -> Self::Gradient {
        if self.name() == args.name() {
            return T::one();
        }
        T::zero()
    }
}

unsafe impl<T> Send for Variable<T> {}

unsafe impl<T> Sync for Variable<T> {}

impl<T> Add for Variable<T>
where
    T: Add<Output = T> + Default,
{
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        let name = format!("{} + {}", self.name, rhs.name);
        let value = self.eval() + rhs.eval();
        Variable::new(name).with_value(value)
    }
}

impl<T> Add<T> for Variable<T>
where
    T: Add<Output = T> + Default + std::fmt::Display,
{
    type Output = Self;

    fn add(self, rhs: T) -> Self::Output {
        let name = format!("{} + {}", self.name, rhs);
        let value = self.eval() + rhs;
        Variable::new(name).with_value(value)
    }
}

impl<T> Div for Variable<T>
where
    T: Div<Output = T> + Default,
{
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        let name = format!("{} / {}", self.name, rhs.name);
        let value = self.eval() / rhs.eval();
        Variable::new(name).with_value(value)
    }
}

impl<T> Div<T> for Variable<T>
where
    T: Div<Output = T> + Default + std::fmt::Display,
{
    type Output = Self;

    fn div(self, rhs: T) -> Self::Output {
        let name = format!("{} / {}", self.name, rhs);
        let value = self.eval() / rhs;
        Variable::new(name).with_value(value)
    }
}

impl<T> Mul for Variable<T>
where
    T: Mul<Output = T> + Default,
{
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        let name = format!("{} * {}", self.name, rhs.name);
        let value = self.eval() * rhs.eval();
        Variable::new(name).with_value(value)
    }
}

impl<T> Mul<T> for Variable<T>
where
    T: Mul<Output = T> + Default + std::fmt::Display,
{
    type Output = Self;

    fn mul(self, rhs: T) -> Self::Output {
        let name = format!("{} * {}", self.name, rhs);
        let value = self.eval() * rhs;
        Variable::new(name).with_value(value)
    }
}

impl<T> Sub for Variable<T>
where
    T: Sub<Output = T> + Default,
{
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        let name = format!("{} - {}", self.name, rhs.name);
        let value = self.eval() - rhs.eval();
        Variable::new(name).with_value(value)
    }
}

impl<T> Sub<T> for Variable<T>
where
    T: Sub<Output = T> + Default + std::fmt::Display,
{
    type Output = Self;

    fn sub(self, rhs: T) -> Self::Output {
        let name = format!("{} - {}", self.name, rhs);
        let value = self.eval() - rhs;
        Variable::new(name).with_value(value)
    }
}

impl<T> One for Variable<T>
where
    T: Clone + Default + One,
{
    fn one() -> Self {
        Variable::new("one").with_value(T::one())
    }
}

impl<T> Zero for Variable<T>
where
    T: Clone + Default + Zero,
{
    fn zero() -> Self {
        Variable::new("0").with_value(T::zero())
    }

    fn is_zero(&self) -> bool {
        self.clone().eval().is_zero()
    }
}
