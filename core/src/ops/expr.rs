/*
    Appellation: expr <mod>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
use crate::id::IndexId;
use crate::ops::{BinaryOp, TernaryOp, UnaryOp};
use crate::prelude::AnyBox;
use paste::paste;
use strum::EnumIs;

#[derive(Clone, Debug, EnumIs, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum Expr<K = usize, V = AnyBox> {
    Binary(ExprBinary<K, V>),
    Unary(ExprUnary<K, V>),

    Constant(V),
    Variable { id: IndexId<K>, value: V },
}

impl<K, V> Expr<K, V> {
    pub fn binary(lhs: Expr<K, V>, rhs: Expr<K, V>, op: BinaryOp) -> Self {
        Self::Binary(ExprBinary::new(lhs, rhs, op))
    }

    pub fn constant(value: V) -> Self {
        Self::Constant(value)
    }

    pub fn unary(recv: Expr<K, V>, op: UnaryOp) -> Self {
        Self::Unary(ExprUnary::new(recv, op))
    }

    pub fn variable(idx: K, value: V) -> Self {
        Self::Variable {
            id: IndexId::from_index(idx),
            value,
        }
    }
}

macro_rules! expr_variant {
    ($variant:ident<$op:ty>($($param:ident),*)) => {
        #[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
        pub struct $variant<K = usize, V = AnyBox> {
            op: $op,
            $($param: Box<Expr<K, V>>),*
        }

        impl<K, V> $variant<K, V> {
            pub fn new($($param: Expr<K, V>,)* op: $op,) -> Self {
                Self {
                    op,
                    $($param: Box::new($param)),*
                }
            }

            pub fn op(&self) -> $op {
                self.op
            }

            pub fn op_mut(&mut self) -> &mut $op {
                &mut self.op
            }

            $(
                pub fn $param(&self) -> &Expr<K, V> {
                    &self.$param
                }
            )*

            paste! {
                $(
                    pub fn [<$param _mut>](&mut self) -> &mut Expr<K, V> {
                        &mut self.$param
                    }
                )*
            }
        }
    };

}

expr_variant!(ExprBinary<BinaryOp>(lhs, rhs));
expr_variant!(ExprTernary<TernaryOp>(x, y, z));
expr_variant!(ExprUnary<UnaryOp>(recv));
