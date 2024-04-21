/*
   Appellation: operator <mod>
   Contrib: FL03 <jo3mccain@icloud.com>
*/
use super::{BinArgs, BinaryOp};
pub struct BinaryOperator<Args>
where
    Args: BinArgs,
{
    pub args: Args,
    pub communitative: bool,
    pub op: BinaryOp,
}

impl<Args> BinaryOperator<Args>
where
    Args: BinArgs,
{
    pub fn new(args: Args, op: BinaryOp) -> Self {
        Self {
            args,
            communitative: op.is_commutative(),
            op,
        }
    }

    pub fn lhs(&self) -> &Args::Lhs {
        self.args.lhs()
    }

    pub fn rhs(&self) -> &Args::Rhs {
        self.args.rhs()
    }

    pub fn args(&self) -> &Args {
        &self.args
    }
}
