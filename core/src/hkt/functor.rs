/*
    Appellation: functor <mod>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
//! # Functor
//!
//! A functor is a type that when mapped over, preserves the structure of the type while applying a function to the values within the type.
//! Functors are useful for modeling the functional effects on values of parameterized data types.
use super::HKT;
use std::rc::Rc;
use std::sync::Arc;

pub trait Functor<U>: HKT<U> {
    fn fmap<F>(&self, f: F) -> Self::T
    where
        F: Fn(&Self::C) -> U;
}

impl<T, U> Functor<U> for Arc<T> {
    fn fmap<F>(&self, f: F) -> Arc<U>
    where
        F: Fn(&T) -> U,
    {
        Arc::new(f(self))
    }
}

impl<T, U> Functor<U> for Box<T> {
    fn fmap<F>(&self, f: F) -> Box<U>
    where
        F: Fn(&T) -> U,
    {
        Box::new(f(self))
    }
}

impl<T, U> Functor<U> for Option<T> {
    fn fmap<F>(&self, f: F) -> Option<U>
    where
        F: Fn(&T) -> U,
    {
        if let Some(ref value) = self {
            return Some(f(value));
        }
        None
    }
}

impl<T, U> Functor<U> for Rc<T> {
    fn fmap<F>(&self, f: F) -> Rc<U>
    where
        F: Fn(&T) -> U,
    {
        Rc::new(f(self))
    }
}

impl<T, U> Functor<U> for Vec<T> {
    fn fmap<F>(&self, f: F) -> Vec<U>
    where
        F: Fn(&T) -> U,
    {
        let mut result = Vec::with_capacity(self.len());
        for value in self {
            result.push(f(value));
        }
        result
    }
}
