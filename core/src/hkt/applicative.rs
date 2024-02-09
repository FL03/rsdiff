/*
    Appellation: applicative <mod>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
use super::functor::Functor;
use super::HKT;

use std::rc::Rc;

pub trait Applicative<U>: Functor<U> {
    fn pure_(value: U) -> Self::T
    where
        Self: HKT<U, C = U>;
    fn seq<F>(&self, fs: <Self as HKT<F>>::T) -> <Self as HKT<U>>::T
    where
        F: Fn(&<Self as HKT<U>>::C) -> U,
        Self: HKT<F>;
}

impl<T, U> Applicative<U> for Box<T> {
    fn pure_(value: U) -> Self::T {
        Box::new(value)
    }

    fn seq<F>(&self, fs: <Self as HKT<F>>::T) -> Box<U>
    where
        F: Fn(&T) -> U,
    {
        let v = fs(self);
        Box::new(v)
    }
}

impl<T, U> Applicative<U> for Option<T> {
    fn pure_(value: U) -> Self::T {
        Some(value)
    }

    fn seq<F>(&self, fs: <Self as HKT<F>>::T) -> Option<U>
    where
        F: Fn(&T) -> U,
    {
        match *self {
            Some(ref value) => match fs {
                Some(f) => Some(f(value)),
                None => None,
            },
            None => None,
        }
    }
}

impl<T, U> Applicative<U> for Rc<T> {
    fn pure_(value: U) -> Self::T {
        Rc::new(value)
    }

    fn seq<F>(&self, fs: <Self as HKT<F>>::T) -> Rc<U>
    where
        F: Fn(&T) -> U,
    {
        let v = fs(self);
        Rc::new(v)
    }
}

impl<T, U> Applicative<U> for Vec<T> {
    fn pure_(value: U) -> Self::T {
        vec![value]
    }

    fn seq<F>(&self, fs: <Self as HKT<F>>::T) -> Vec<U>
    where
        F: Fn(&T) -> U,
    {
        let mut result = Vec::new();
        for (i, f) in fs.into_iter().enumerate() {
            let v = (f)(&self[i]);
            result.push(v)
        }
        return result;
    }
}
