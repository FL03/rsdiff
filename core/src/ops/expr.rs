/*
    Appellation: expr <mod>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
use crate::id::IndexId;
use crate::ops::{BinaryOp, NaryOp, TernaryOp, UnaryOp};
use crate::prelude::AnyBox;
use paste::paste;
use strum::EnumIs;

pub(crate) type BoxExpr<K = usize, V = AnyBox> = Box<Expr<K, V>>;

#[derive(Clone, Debug, EnumIs, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum Expr<K = usize, V = AnyBox> {
    Binary(ExprBinary<K, V>),
    Nary(ExprNary<K, V>),
    Ternary(ExprTernary<K, V>),
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

    pub fn nary(args: impl IntoIterator<Item = Expr<K, V>>, op: NaryOp) -> Self {
        Self::Nary(ExprNary::new(args, op))
    }

    pub fn ternary(x: Expr<K, V>, y: Expr<K, V>, z: Expr<K, V>, op: TernaryOp) -> Self {
        Self::Ternary(ExprTernary::new(x, y, z, op))
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

    pub fn boxed(self) -> BoxExpr<K, V> {
        Box::new(self)
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

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct ExprNary<K = usize, V = AnyBox> {
    args: Vec<Box<Expr<K, V>>>,
    op: NaryOp,
}

impl<K, V> ExprNary<K, V> {
    pub fn new(args: impl IntoIterator<Item = Expr<K, V>>, op: NaryOp) -> Self {
        Self {
            args: Vec::from_iter(args.into_iter().map(|i| i.boxed())),
            op,
        }
    }

    pub fn as_slice(&self) -> &[Box<Expr<K, V>>] {
        &self.args
    }

    pub fn as_mut_slice(&mut self) -> &mut [Box<Expr<K, V>>] {
        &mut self.args
    }

    pub fn args(&self) -> &Vec<Box<Expr<K, V>>> {
        &self.args
    }

    pub fn args_mut(&mut self) -> &mut Vec<Box<Expr<K, V>>> {
        &mut self.args
    }

    pub fn op(&self) -> NaryOp {
        self.op
    }

    pub fn op_mut(&mut self) -> &mut NaryOp {
        &mut self.op
    }
}
