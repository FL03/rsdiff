/*
    Appellation: monad <mod>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
use super::applicative::Applicative;
use super::HKT;

use std::rc::Rc;
use std::sync::Arc;

pub trait Monad<U>: Applicative<U> {
    fn return_(x: U) -> Self::T
    where
        Self: HKT<U, C = U>,
    {
        Self::pure_(x)
    }

    fn bind<F>(&self, fs: F) -> Self::T
    where
        F: FnMut(&Self::C) -> Self::T;

    fn join<T>(&self) -> T
    where
        Self: HKT<U, T = T, C = T>,
        T: Clone,
    {
        self.bind(|x| x.clone())
    }
}

impl<T, U> Monad<U> for Arc<T> {
    fn bind<F>(&self, mut fs: F) -> Arc<U>
    where
        F: FnMut(&T) -> Arc<U>,
    {
        fs(self)
    }
}

impl<T, U> Monad<U> for Box<T> {
    fn bind<F>(&self, mut fs: F) -> Box<U>
    where
        F: FnMut(&T) -> Box<U>,
    {
        fs(self)
    }
}

impl<T, U> Monad<U> for Option<T> {
    fn bind<F>(&self, mut fs: F) -> Option<U>
    where
        F: FnMut(&T) -> Option<U>,
    {
        match *self {
            Some(ref value) => fs(value),
            None => None,
        }
    }
}

impl<T, U> Monad<U> for Rc<T> {
    fn bind<F>(&self, mut fs: F) -> Rc<U>
    where
        F: FnMut(&T) -> Rc<U>,
    {
        fs(self)
    }
}

impl<T, U> Monad<U> for Vec<T> {
    fn bind<F>(&self, mut fs: F) -> Vec<U>
    where
        F: FnMut(&T) -> Vec<U>,
    {
        let mut v = Vec::new();
        for x in self {
            v.extend(fs(x));
        }
        v
    }
}
