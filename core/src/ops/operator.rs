/*
    Appellation: operator <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
use super::kinds::OpKind;

pub trait OperandType {
    private!();
}

pub trait Operator {
    fn kind(&self) -> OpKind;

    fn name(&self) -> &str;
}


pub trait Params {
    type Pattern;

    fn into_pattern(self) -> Self::Pattern;
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

/*
    **************** implementations ****************
*/
impl Operator for Box<dyn Operator> {
    fn kind(&self) -> OpKind {
        self.as_ref().kind()
    }

    fn name(&self) -> &str {
        self.as_ref().name()
    }
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
args_impl!((A, B), (A, B, C), (A, B, C, D), (A, B, C, D, E));

macro_rules! impl_operand_ty {
    ($($kind:ident),*) => {
        $(
            impl_operand_ty!(@impl $kind);
        )*
    };
    (@impl $kind:ident) => {
        #[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
        #[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
        pub struct $kind;

        impl OperandType for $kind {
            seal!();
        }
    };
}



impl_operand_ty!(Binary, Nary, Ternary, Unary);