/*
    Appellation: variables <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
use crate::prelude::{Evaluate, Gradient};
use num::{Num, One, Zero};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use std::borrow::{Borrow, BorrowMut};

#[cfg_attr(feature = "serde", derive(Deserialize, Serialize,))]
#[derive(Clone, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[repr(C)]
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

    pub const fn is_initialized(&self) -> bool {
        self.value.is_some()
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

    pub fn with_name(mut self, name: impl ToString) -> Self {
        self.name = name.to_string();
        self
    }

    pub fn with_value(mut self, value: T) -> Self {
        self.value = Some(value);
        self
    }
}

impl<T> Borrow<T> for Variable<T> {
    fn borrow(&self) -> &T {
        self.value.as_ref().unwrap()
    }
}

impl<T> BorrowMut<T> for Variable<T> {
    fn borrow_mut(&mut self) -> &mut T {
        self.value.as_mut().unwrap()
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

macro_rules! impl_std_op {
    ($parent:ident: $trait:ident, $method:ident) => {
        impl<T> std::ops::$trait for $parent<T>
        where
            T: Clone + Default + std::ops::$trait<Output = T>,
        {
            type Output = Self;

            fn $method(self, rhs: Self) -> Self::Output {
                let name = format!("{}", stringify!($method));
                let value = self.eval().$method(rhs.eval());
                $parent::new(name).with_value(value)
            }
        }

        impl<T> std::ops::$trait<T> for $parent<T>
        where
            T: Clone + Default + std::ops::$trait<Output = T>,
        {
            type Output = Self;

            fn $method(self, rhs: T) -> Self::Output {
                let name = format!("{}", stringify!($method));
                let value = self.eval().$method(rhs);
                $parent::new(name).with_value(value)
            }
        }
    };
}
impl_std_op!(Variable: Add, add);
impl_std_op!(Variable: Div, div);
impl_std_op!(Variable: Mul, mul);
impl_std_op!(Variable: Sub, sub);
