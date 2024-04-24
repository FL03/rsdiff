/*
    Appellation: operand <traits>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
use crate::ops::{OpKind, Operator};

pub trait OperandType {
    fn kind(&self) -> OpKind;

    private!();
}

pub trait Operand {
    type Kind: OperandType;

    fn kind(&self) -> OpKind {
        self.optype().kind()
    }

    fn name(&self) -> &str;

    fn optype(&self) -> Self::Kind;
}

/*
 **************** implementations ****************
*/
impl<K, S> Operator for S
where
    K: OperandType,
    S: Operand<Kind = K>,
{
    fn kind(&self) -> OpKind {
        self.optype().kind()
    }

    fn name(&self) -> &str {
        Operand::name(self)
    }
}

impl OperandType for Box<dyn OperandType> {
    fn kind(&self) -> OpKind {
        self.as_ref().kind()
    }

    seal!();
}

impl<K> Operand for Box<dyn Operand<Kind = K>>
where
    K: OperandType,
{
    type Kind = K;

    fn name(&self) -> &str {
        self.as_ref().name()
    }

    fn optype(&self) -> Self::Kind {
        self.as_ref().optype()
    }
}

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
            fn kind(&self) -> OpKind {
                OpKind::$kind
            }

            seal!();
        }
    };
}

impl_operand_ty!(Binary, Nary, Ternary, Unary);
