/*
    Appellation: num <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
pub use self::{traits::*, utils::*};

pub mod traits;

pub(crate) mod utils {
    use num::integer::Integer;
    use num::traits::{Num, ToPrimitive, Unsigned};

    pub fn harmonic<T>(n: T) -> f64
    where
        T: Integer + Num + ToPrimitive + Unsigned,
    {
        (1..=n.to_usize().unwrap()).fold(0f64, |acc, i| acc + i.to_f64().unwrap().recip())
    }
}

#[cfg(test)]
mod tests {

    #[cfg(feature = "approx")]
    #[test]
    fn test_harmonic() {
        use approx::assert_relative_eq;
        use super::traits::NaturalNumber;
        assert_relative_eq!(1u8.harmonic(), 1.0);
        assert_relative_eq!(2u16.harmonic(), super::harmonic(2u16));
        assert_relative_eq!(3u32.harmonic(), 11.0 / 6.0);
        assert_relative_eq!(4u64.harmonic(), 25. / 12.);
        assert_relative_eq!(5u128.harmonic(), 137. / 60.);
        assert_relative_eq!(6usize.harmonic(), 2.4499999999999997);
    }
}
