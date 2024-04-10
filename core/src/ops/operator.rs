/*
    Appellation: operator <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/

pub enum OperatorKind {
    Binary,
    Unary,
    Ternary,
    Nary,
}

pub trait Operator {
    fn kind(&self) -> OperatorKind;

    fn name(&self) -> &str;
}

#[allow(dead_code)]
pub(crate) struct Operand {
    kind: OperatorKind,
    name: String,
}

pub trait Args {
    type Pattern;

    fn args(self) -> Self::Pattern;
}

impl Args for () {
    type Pattern = ();

    fn args(self) -> Self::Pattern {
        ()
    }
}

impl<A> Args for (A,) {
    type Pattern = (A,);

    fn args(self) -> Self::Pattern {
        self
    }
}

impl<A, B> Args for (A, B) {
    type Pattern = (A, B);

    fn args(self) -> Self::Pattern {
        self
    }
}

pub trait Evaluator<Args> {
    type Output;

    fn eval(&self, args: Args) -> Self::Output;
}
