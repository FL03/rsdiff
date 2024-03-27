/*
    Appellation: grad <mod>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
use crate::actions::grad::GradStore;
use crate::prelude::{Scalar, TensorId, TensorOp, TensorResult};
use crate::TensorBase;
use acme::prelude::{BinaryOp, Store};
use std::collections::HashMap;
use std::ops::{Add, Mul};

// The vec of sorted nodes is passed as an owned value rather than a mutable reference
// to get around some lifetime limitations.
fn walk<'a, T>(
    node: &'a TensorBase<T>,
    nodes: Vec<&'a TensorBase<T>>,
    visited: &mut HashMap<TensorId, bool>,
) -> (bool, Vec<&'a TensorBase<T>>) {
    if let Some(&tg) = visited.get(&node.id()) {
        return (tg, nodes);
    }
    // track the gradient of the current node
    let mut track = false;
    let mut nodes = if node.is_variable() {
        // Do not call recursively on the "leaf" nodes.
        track = true;
        nodes
    } else if let Some(op) = node.op() {
        match op {
            TensorOp::Binary(lhs, rhs, _kind) => {
                let (tg, nodes) = walk(lhs, nodes, visited);
                track |= tg;
                let (tg, nodes) = walk(rhs, nodes, visited);
                track |= tg;
                nodes
            }
            _ => nodes,
        }
    } else {
        nodes
    };
    visited.insert(node.id(), track);
    if track {
        nodes.push(node);
    }
    (track, nodes)
}

impl<T> TensorBase<T>
where
    T: Scalar,
{
    fn sorted_nodes(&self) -> Vec<&TensorBase<T>> {
        let (_tg, mut nodes) = walk(self, vec![], &mut HashMap::new());
        nodes.reverse();
        nodes
    }

    pub fn grad(&self) -> TensorResult<GradStore<T>>
    where
        T: std::fmt::Debug,
    {
        // get the sorted nodes
        let sorted = self.sorted_nodes();
        // initialize a new gradient store
        let mut store = GradStore::new();
        // insert the gradient w.r.t. the current node
        store.insert(self.id(), self.ones_like());

        for node in sorted {
            if node.is_variable() {
                continue;
            }
            // get the gradient of the node
            let grad = store.remove(&node.id()).expect("Gradient not found");
            let grad = grad.detach();
            // handle the different types of operations
            if let Some(op) = &self.op {
                match op {
                    TensorOp::Binary(lhs, rhs, kind) => match kind {
                        BinaryOp::Add => {
                            let a = store.entry(lhs.id()).or_insert(lhs.zeros_like());
                            *a = &*a + &grad;
                            let b = store.entry(rhs.id()).or_insert(rhs.zeros_like());
                            *b = &*b + &grad;
                        }
                        BinaryOp::Mul => {
                            *store.entry(lhs.id()).or_insert(lhs.zeros_like()) +=
                                &grad * rhs.as_ref();
                            *store.entry(rhs.id()).or_insert(rhs.zeros_like()) +=
                                &grad * lhs.as_ref();
                        }
                        _ => todo!(),
                    },
                    TensorOp::Unary(_a, kind) => match kind {
                        _ => todo!(),
                    },
                    _ => {}
                }
            }
        }

        Ok(store)
    }
}
