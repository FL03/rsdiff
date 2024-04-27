/*
    Appellation: num <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
use paste::paste;

pub trait NaturalNumber {
    fn harmonic(&self) -> f64;
}

macro_rules! natural {
    ($($n:tt),*) => {
        $(natural!(@impl $n);)*
    };
    (@impl $n:tt) => {
        paste! {
            impl NaturalNumber for [<u$n>] {
                fn harmonic(&self) -> f64 {
                    let mut sum = 0.0;
                    for i in 1..=*self {
                        sum += (i as f64).recip();
                    }
                    sum
                }
            }

            impl NaturalNumber for core::num::[<NonZeroU$n>] {
                fn harmonic(&self) -> f64 {
                    self.get().harmonic()
                }
            }
        }
    };
}

natural!(8, 16, 32, 64, 128, size);
