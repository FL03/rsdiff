/*
    Appellation: approx <impls>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
#![cfg(feature = "approx")]
use crate::TensorBase;
use ndarray::{ArrayBase, Data, Dimension};

use approx::{AbsDiffEq, RelativeEq};

impl<A, S, D> AbsDiffEq for TensorBase<S, D>
where
    A: AbsDiffEq<Epsilon = A>,
    D: Dimension,
    S: Data<Elem = A>,
    ArrayBase<S, D>: AbsDiffEq<Epsilon = A>,
{
    type Epsilon = A;

    fn default_epsilon() -> A {
        A::default_epsilon()
    }

    fn abs_diff_eq(&self, other: &Self, epsilon: A) -> bool {
        self.data().abs_diff_eq(&other.data, epsilon)
    }
}

impl<A, S, D> AbsDiffEq<ArrayBase<S, D>> for TensorBase<S, D>
where
    A: AbsDiffEq<Epsilon = A>,
    D: Dimension,
    S: Data<Elem = A>,
    ArrayBase<S, D>: AbsDiffEq<Epsilon = A>,
{
    type Epsilon = A;

    fn default_epsilon() -> A {
        A::default_epsilon()
    }

    fn abs_diff_eq(&self, other: &ArrayBase<S, D>, epsilon: A) -> bool {
        self.data().abs_diff_eq(&other, epsilon)
    }
}

impl<A, S, D> RelativeEq for TensorBase<S, D>
where
    A: RelativeEq<Epsilon = A>,
    D: Dimension,
    S: Data<Elem = A>,
    ArrayBase<S, D>: RelativeEq<Epsilon = A>,
{
    fn default_max_relative() -> A {
        A::default_max_relative()
    }

    fn relative_eq(&self, other: &Self, epsilon: A, max_relative: A) -> bool {
        self.data().relative_eq(&other.data, epsilon, max_relative)
    }
}

impl<A, S, D> RelativeEq<ArrayBase<S, D>> for TensorBase<S, D>
where
    A: RelativeEq<Epsilon = A>,
    D: Dimension,
    S: Data<Elem = A>,
    ArrayBase<S, D>: RelativeEq<Epsilon = A>,
{
    fn default_max_relative() -> A {
        A::default_max_relative()
    }

    fn relative_eq(&self, other: &ArrayBase<S, D>, epsilon: A, max_relative: A) -> bool {
        self.data().relative_eq(&other, epsilon, max_relative)
    }
}
