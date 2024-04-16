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
    () => {
        impl Params for () {
            type Pattern = ();

            fn into_pattern(self) -> Self::Pattern {
                ()
            }
        }

        impl Params for [(); 0] {
            type Pattern = ();

            fn into_pattern(self) -> Self::Pattern {
                ()
            }
        }
    };
    ($n:ident) => {
        impl<$n> Params for ($n,) {
            type Pattern = ($n,);

            fn into_pattern(self) -> Self::Pattern {
                self
            }
        }

        impl<$n> Params for [$n; 1] where $n: Copy {
            type Pattern = ($n,);

            fn into_pattern(self) -> Self::Pattern {
                (self[0],)
            }
        }
    };
    ($($n:tt),*) => {
        args_impl!(@loop $($n),*);
    };
    (@loop $(($($n:ident),*)),*) => {
        $(
            args_impl!(@loop $($n),*);
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
}

args_impl!();
args_impl!(A);
args_impl!((A, B), (A, B, C), (A, B, C, D));

pub struct Adder;

impl Operator for Adder {
    fn kind(&self) -> OpKind {
        OpKind::Binary
    }

    fn name(&self) -> &str {
        "adder"
    }
}

impl<A, B, C> super::binary::BinOp<A, B> for Adder
where
    A: core::ops::Add<B, Output = C>,
{
    type Output = C;

    fn eval(&self, lhs: A, rhs: B) -> Self::Output {
        lhs + rhs
    }
}

impl<P, A, B, C> Evaluator<P> for Adder
where
    A: core::ops::Add<B, Output = C>,
    P: Params<Pattern = (A, B)>,
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
    P: Params<Pattern = (A, B)>,
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

// impl<Args, C> Evaluator<Args> for Box<dyn Evaluator<Args, Output = C> + Operator> where Args: Params {
//     type Output = C;

//     fn eval(&self, args: Args) -> Self::Output {
//         self.as_ref().eval(args)
//     }
// }

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
        let args = ();
        let pattern = args.into_pattern();
        assert_eq!(pattern, args);
        let args = (10f64,);
        let pattern = args.into_pattern();
        assert_eq!(pattern, args);
        let args = (0f64, 0f32);
        let pattern = args.into_pattern();
        assert_eq!(pattern, args);
        let args = (0f64, 0f32, 0usize);
        let pattern = args.into_pattern();
        assert_eq!(pattern, args);
    }
}
