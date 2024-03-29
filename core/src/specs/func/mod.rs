/*
    Appellation: func <mod>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
pub use self::structural::*;

pub(crate) mod structural;

pub trait FnHandler<Args> {
    type Output;

    fn item_fn(&self) -> fn(Args) -> Self::Output;
}

#[allow(unused)]
#[cfg(test)]
mod tests {
    use super::FnHandler;
    use core::ops::Mul;

    pub struct Sample;

    impl Sample {
        pub fn sqr<T>(x: T) -> T
        where
            T: Copy + Mul<T, Output = T>,
        {
            x * x
        }

        pub fn blahblah<T>() -> fn(T) -> T
        where
            T: Copy + Mul<T, Output = T>,
        {
            Sample::sqr
        }
    }

    impl<T> FnHandler<T> for Sample
    where
        T: Copy + Mul<T, Output = T>,
    {
        type Output = T;

        fn item_fn(&self) -> fn(T) -> T {
            Self::sqr
        }
    }

    #[test]
    fn test_fn_handler() {
        let sample = Sample;
        let item_fn = sample.item_fn();
        assert_eq!(item_fn(2), 4);
    }
}
