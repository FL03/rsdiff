/*
    Appellation: scalar <mod>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
use core::iter::{Product, Sum};
use core::ops::Neg;
use num::complex::Complex;
use num::traits::{Float, FromPrimitive, Inv, NumAssign, NumCast, NumOps, Pow};

pub trait Scalar:
    Copy
    + Default
    + FromPrimitive
    + Inv<Output = Self>
    + Neg<Output = Self>
    + NumAssign
    + NumCast
    + NumOps
    + Pow<Self, Output = Self>
    + Product
    + Sum
    + 'static
{
    type Complex: Scalar<Complex = Self::Complex, Real = Self::Real>
        + NumOps<Self::Real, Self::Complex>;
    type Real: Scalar<Complex = Self::Complex, Real = Self::Real>
        + NumOps<Self::Complex, Self::Complex>;

    fn from_real(re: Self::Real) -> Self;

    fn abs(self) -> Self::Real;

    fn add_complex(&self, other: Self::Complex) -> Self::Complex {
        self.as_complex() + other
    }

    fn div_complex(&self, other: Self::Complex) -> Self::Complex {
        self.as_complex() / other
    }

    fn mul_complex(&self, other: Self::Complex) -> Self::Complex {
        self.as_complex() * other
    }

    fn sub_complex(&self, other: Self::Complex) -> Self::Complex {
        self.as_complex() - other
    }

    fn as_complex(&self) -> Self::Complex;

    fn conj(&self) -> Self::Complex;

    fn im(&self) -> Self::Real {
        Default::default()
    }

    fn re(&self) -> Self::Real;

    fn cos(self) -> Self;

    fn cosh(self) -> Self;

    fn exp(self) -> Self;

    fn inv(self) -> Self {
        Inv::inv(self)
    }

    fn ln(self) -> Self;

    fn pow(self, exp: Self) -> Self;

    fn powc(self, exp: Self::Complex) -> Self::Complex;

    fn powf(self, exp: Self::Real) -> Self;

    fn powi(self, exp: i32) -> Self {
        let exp = Self::Real::from_i32(exp).unwrap();
        self.powf(exp)
    }

    fn recip(self) -> Self {
        Self::one() / self
    }

    fn sin(self) -> Self;

    fn sinh(self) -> Self;

    fn sqrt(self) -> Self;

    fn sqr(self) -> Self {
        self.powi(2)
    }

    fn tan(self) -> Self;

    fn tanh(self) -> Self;
}

impl<T> Scalar for Complex<T>
where
    T: Scalar<Complex = Self, Real = T>,
    T::Real: NumOps<Complex<T>, Complex<T>> + Float,
{
    type Complex = Self;
    type Real = T;

    fn from_real(re: Self::Real) -> Self {
        Complex::new(re, Default::default())
    }

    fn abs(self) -> Self::Real {
        Complex::norm(self)
    }

    fn as_complex(&self) -> Self::Complex {
        *self
    }

    fn conj(&self) -> Self::Complex {
        Complex::conj(self)
    }

    fn re(&self) -> Self::Real {
        self.re
    }

    fn im(&self) -> Self::Real {
        self.im
    }

    fn cos(self) -> Self {
        Complex::cos(self)
    }

    fn cosh(self) -> Self {
        Complex::cosh(self)
    }

    fn exp(self) -> Self {
        Complex::exp(self)
    }

    fn ln(self) -> Self {
        Complex::ln(self)
    }

    fn pow(self, exp: Self) -> Self {
        Complex::powc(self, exp)
    }

    fn powc(self, exp: Self::Complex) -> Self::Complex {
        Complex::powc(self, exp)
    }

    fn powf(self, exp: T) -> Self {
        Complex::powf(self, exp)
    }

    fn powi(self, exp: i32) -> Self {
        Complex::powi(&self, exp)
    }

    fn sin(self) -> Self {
        Complex::sin(self)
    }

    fn sinh(self) -> Self {
        Complex::sinh(self)
    }

    fn sqrt(self) -> Self {
        Complex::sqrt(self)
    }

    fn tan(self) -> Self {
        Complex::tan(self)
    }

    fn tanh(self) -> Self {
        Complex::tanh(self)
    }
}

macro_rules! impl_scalar {
    ($re:ty) => {
        impl Scalar for $re {
            type Complex = Complex<$re>;
            type Real = $re;

            fn from_real(re: Self::Real) -> Self {
                re
            }

            fn abs(self) -> Self::Real {
                <$re>::abs(self)
            }

            fn as_complex(&self) -> Self::Complex {
                Complex::new(*self, <$re>::default())
            }

            fn conj(&self) -> Self::Complex {
                Complex::new(*self, -<$re>::default())
            }

            fn re(&self) -> Self::Real {
                *self
            }

            fn cos(self) -> Self {
                <$re>::cos(self)
            }

            fn cosh(self) -> Self {
                <$re>::cosh(self)
            }

            fn exp(self) -> Self {
                <$re>::exp(self)
            }

            fn ln(self) -> Self {
                <$re>::ln(self)
            }

            fn pow(self, exp: Self) -> Self {
                <$re>::powf(self, exp)
            }

            fn powc(self, exp: Self::Complex) -> Self::Complex {
                Complex::new(self, <$re>::default()).powc(exp)
            }

            fn powf(self, exp: Self::Real) -> Self {
                <$re>::powf(self, exp)
            }

            fn powi(self, exp: i32) -> Self {
                <$re>::powi(self, exp)
            }

            fn sin(self) -> Self {
                <$re>::sin(self)
            }

            fn sinh(self) -> Self {
                <$re>::sinh(self)
            }

            fn sqrt(self) -> Self {
                <$re>::sqrt(self)
            }

            fn tan(self) -> Self {
                <$re>::tan(self)
            }

            fn tanh(self) -> Self {
                <$re>::tanh(self)
            }
        }
    };
}

impl_scalar!(f32);
impl_scalar!(f64);
