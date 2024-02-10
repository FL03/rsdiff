/*
    Appellation: scalar <mod>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
use num::complex::Complex;
use num::traits::real::Real;
use num::traits::{Float, FromPrimitive, NumAssign, NumCast, NumOps};
use std::iter::{Product, Sum};
use std::ops::Neg;

pub trait Scalar:
    Copy + Default + FromPrimitive + Neg<Output = Self> + NumAssign + NumCast + NumOps + Product + Sum + 'static
{
    type Complex: Scalar<Complex = Self::Complex, Real = Self::Real> + NumOps<Self::Real>;
    type Real: Scalar<Complex = Self::Complex, Real = Self::Real>
        + NumOps<Self::Complex, Self::Complex>
        + Real;

    fn abs(self) -> Self::Real {
        let re = self.real();
        let im = self.imag();
        <<Self as Scalar>::Real as Real>::sqrt(re * re + im * im)
    }

    fn conj(&self) -> Self::Complex;

    fn imag(&self) -> Self::Real {
        Default::default()
    }

    fn real(&self) -> Self::Real;



    fn sqrt(self) -> Self;
}


impl<T> Scalar for Complex<T>
where
    T: Scalar<Complex = Self, Real = T>,
    T::Real: NumOps<Complex<T>, Complex<T>> + Float,
{
    type Complex = Self;
    type Real = T;

    fn conj(&self) -> Self::Complex {
        Complex::new(self.re, -self.im)
    }

    fn real(&self) -> Self::Real {
        self.re
    }

    fn imag(&self) -> Self::Real {
        self.im
    }

    fn sqrt(self) -> Self {
        Complex::sqrt(self)
    }
}

macro_rules! impl_scalar {
    ($t:ty) => {
        impl Scalar for $t {
            type Complex = Complex<$t>;
            type Real = $t;

            fn conj(&self) -> Self::Complex {
                Complex::new(*self, 0.0)
            }

            fn real(&self) -> Self::Real {
                *self
            }

            fn sqrt(self) -> Self {
                <$t>::sqrt(self)
            }
        }
    };
    ($($t:ty),*) => {
        $(
            impl_scalar!($t);
        )*
    };
}
impl_scalar!(f32);
impl_scalar!(f64);