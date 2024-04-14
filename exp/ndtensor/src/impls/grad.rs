/*
    Appellation: grad <impls>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
use crate::nd::{Data, Dimension};
use crate::ops::TensorExpr;
use crate::{ScalarExt, Tensor, TensorBase, TensorId, TensorView};
use std::collections::HashMap;

pub(crate) type Visited<K = TensorId> = HashMap<K, bool>;

macro_rules! entry {
    ($ctx:expr, $entry:expr) => {
        entry!($ctx, $entry, $entry.zeros_like())
    };
    ($ctx:expr, $entry:expr, $default:expr) => {
        $ctx.entry($entry.id()).or_insert($default)
    };
}

impl<A, S> Tensor<S>
where
    A: ScalarExt,
    S: Data<Elem = A>,
{
    /// toposort is a function which sorts the nodes of the op graph in topological order.
    fn toposort(&self, reverse: bool) -> Vec<crate::TensorView<'_, S>> {
        unimplemented!()
    }
}
