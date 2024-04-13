/*
    Appellation: operator <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
use super::kinds::OpKind;

pub trait Operator {
    fn kind(&self) -> OpKind;

    fn name(&self) -> &str;
}

impl Operator for Box<dyn Operator> {
    fn kind(&self) -> OpKind {
        self.as_ref().kind()
    }

    fn name(&self) -> &str {
        self.as_ref().name()
    }
}
pub trait Params {
    type Pattern;

    fn into_pattern(self) -> Self::Pattern;
}

macro_rules! args_impl {
    (nary) => {
        impl Params for () {
            type Pattern = ();

            fn into_pattern(self) -> Self::Pattern {
                ()
            }
        }
    };
    ($($n:tt),*) => {
        impl_args!(@loop $($n),*);
    };
    (b $(($($n:tt),*)),*) => {
        $(
            impl_args!(@loop $($n),*);
        )*
    };
    (@loop $(($($n:ident),*)),*) => {
        impl_args!(@loop $(($($n),*)),*);
    };
    (@loop $($n:ident),*) => {
        impl<$($n),*> Params for ($($n),*) {
            type Pattern = ($($n),*);

            fn into_pattern(self) -> Self::Pattern {
                self
            }
        }
    };
}

macro_rules! impl_args {

    ($($n:ident),*) => {
        impl_args!(@loop $($n),*);
    };
    (alt: $(($($n:ident),*)),*) => {
        $(
            impl_args!(@loop $($n),*);
        )*
    };
    (@loop $($n:ident),*) => {
        impl<$($n),*> Params for ($($n),*) {
            type Pattern = ($($n),*);

            fn into_pattern(self) -> Self::Pattern {
                self
            }
        }
    };
    (@loop $($n:ident),*) => {
        $($n),*
    };
}
args_impl!(nary);
// args_impl!(A);
args_impl!(A, B);
impl_args!(A, B, C);

// impl<A> Args for (A,) {
//     type Pattern = (A,);

//     fn args(self) -> Self::Pattern {
//         self
//     }
// }

// impl<A, B> Args for (A, B) {
//     type Pattern = (A, B);

//     fn args(self) -> Self::Pattern {
//         self
//     }
// }

pub struct Adder;

impl Operator for Adder {
    fn kind(&self) -> OpKind {
        OpKind::Binary
    }

    fn name(&self) -> &str {
        "adder"
    }
}

impl<P, A, B, C> Evaluator<P> for Adder
where
    A: core::ops::Add<B, Output = C>,
    P: super::binary::BinArgs<Lhs = A, Rhs = B>,
{
    type Output = C;

    fn eval(&self, args: P) -> Self::Output {
        let args = args.into_pattern();
        args.0 + args.1
    }
}

impl<P, A, B, C> Differentiable<P> for Adder
where
    A: core::ops::Add<B, Output = C>,
    P: super::binary::BinArgs<Lhs = A, Rhs = B>,
{
    type Grad = C;

    fn grad(&self, args: P) -> Self::Grad {
        let args = args.into_pattern();
        args.0 + args.1
    }
}

pub trait Evaluator<Args>
where
    Self: Operator,
    Args: Params,
{
    type Output;

    fn eval(&self, args: Args) -> Self::Output;
}

pub trait Differentiable<Args>
where
    Self: Evaluator<Args>,
    Args: Params,
{
    type Grad;

    fn grad(&self, args: Args) -> Self::Grad;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_args() {
        let args = (0f64, 0f32);
        let pattern = args.into_pattern();
        assert_eq!(pattern, args);
        let args = (0f64, 0f32, 0usize);
        let pattern = args.into_pattern();
        assert_eq!(pattern, args);
    }
}
