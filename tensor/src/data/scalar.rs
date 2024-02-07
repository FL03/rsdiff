/*
    Appellation: scalar <mod>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
use num::complex::Complex;
use num::traits::real::Real;
use num::traits::{FromPrimitive, NumAssign, NumCast, NumOps};
use std::iter::{Product, Sum};
use std::ops::Neg;

pub trait Scalar:
    Copy + FromPrimitive + Neg<Output = Self> + NumAssign + NumCast + NumOps + Product + Sum + 'static
{
    type Complex: Scalar<Complex = Self::Complex, Real = Self::Real> + NumOps<Self::Real>;
    type Real: Scalar<Complex = Self::Complex, Real = Self::Real>
        + NumOps<Self::Complex, Self::Complex>
        + Real;

    fn conj(&self) -> Self::Complex;

    fn real(&self) -> Self::Real;

    fn imag(&self) -> Self::Real;
}

impl Scalar for f32 {
    type Complex = Complex<f32>;
    type Real = f32;

    fn conj(&self) -> Self::Complex {
        Complex::new(*self, 0.0)
    }

    fn real(&self) -> Self::Real {
        *self
    }

    fn imag(&self) -> Self::Real {
        0.0
    }
}

impl Scalar for f64 {
    type Complex = Complex<f64>;
    type Real = f64;

    fn conj(&self) -> Self::Complex {
        Complex::new(*self, 0.0)
    }

    fn real(&self) -> Self::Real {
        *self
    }

    fn imag(&self) -> Self::Real {
        0.0
    }
}

impl<T> Scalar for Complex<T>
where
    T: Scalar<Complex = Self, Real = T>,
    T::Real: NumOps<Complex<T>, Complex<T>> + Real,
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
}
