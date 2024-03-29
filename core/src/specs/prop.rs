/*
    Appellation: prop <mod>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
use crate::errors::PredictError;

/// [Backward] describes an object capable of backward propagation.
///
///  
pub trait Backward {
    type Output;

    fn backward(&self) -> Self::Output;
}

/// [Forward] describes an object capable of forward propagation.
pub trait Forward<T> {
    type Output;

    fn forward(&self, args: &T) -> Result<Self::Output, PredictError>;
}

pub trait ForwardIter<T> {
    type Item: Forward<T, Output = T>;

    fn forward_iter(self, args: &T) -> Result<<Self::Item as Forward<T>>::Output, PredictError>;
}

// Trait implementations
mod impls {
    use super::*;

    impl<I, M, T> ForwardIter<T> for I
    where
        I: Iterator<Item = M>,
        M: Forward<T, Output = T>,
        T: Clone,
    {
        type Item = M;

        fn forward_iter(self, args: &T) -> Result<M::Output, PredictError> {
            let mut result = args.clone();
            for i in self {
                result = i.forward(&result)?;
            }
            Ok(result)
        }
    }

    impl<S, T> Forward<T> for Option<S>
    where
        S: Forward<T, Output = T>,
        T: Clone,
    {
        type Output = T;

        fn forward(&self, args: &T) -> Result<Self::Output, PredictError> {
            match self {
                Some(s) => s.forward(args),
                None => Ok(args.clone()),
            }
        }
    }

    impl<S, T> Forward<T> for S
    where
        S: AsRef<dyn Forward<T, Output = T>>,
        T: Clone,
    {
        type Output = T;

        fn forward(&self, args: &T) -> Result<Self::Output, PredictError> {
            self.as_ref().forward(args)
        }
    }
}
