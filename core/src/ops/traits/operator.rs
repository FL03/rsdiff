/*
    Appellation: operator <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
use crate::ops::{OpKind, OperandType};

pub trait Operator {
    fn kind(&self) -> OpKind;

    fn name(&self) -> &str;

    #[doc(hidden)]
    fn optype(&self) -> Box<dyn OperandType> {
        self.kind().optype()
    }
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
