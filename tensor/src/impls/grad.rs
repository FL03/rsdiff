/*
    Appellation: grad <mod>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
use crate::prelude::{Scalar, TensorId, TensorOp};
use crate::tensor::*;
use acme::ops::binary::BinaryOp;
use std::collections::HashMap;
pub(crate) type GradStore<T> = std::collections::BTreeMap<TensorId, T>;

impl<T> TensorBase<T>
where
    T: Scalar,
{
    fn sorted_nodes(&self) -> Vec<&TensorBase<T>> {
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
            let mut track_grad = false;
            let mut nodes = if node.is_variable() {
                // Do not call recursively on the "leaf" nodes.
                track_grad = true;
                nodes
            } else if let Some(op) = node.op() {
                match op {
                    TensorOp::Binary(a, b, _kind) => {
                        let (tg, nodes) = walk(a, nodes, visited);
                        track_grad |= tg;
                        let (tg, nodes) = walk(b, nodes, visited);
                        track_grad |= tg;
                        nodes
                    }
                    _ => nodes,
                }
            } else {
                nodes
            };
            visited.insert(node.id(), track_grad);
            if track_grad {
                nodes.push(node);
            }
            (track_grad, nodes)
        }
        let (_tg, mut nodes) = walk(self, vec![], &mut HashMap::new());
        nodes.reverse();
        nodes
    }

    pub fn grad(&self) -> GradStore<TensorBase<T>>
    where
        T: std::fmt::Debug,
    {
        // get the sorted nodes
        let sorted = self.sorted_nodes();
        // initialize a new gradient store
        let mut store = GradStore::new();
        store.insert(self.id(), self.ones_like());

        for node in sorted.iter() {
            if node.is_variable() {
                continue;
            }
            let grad = store.get(&node.id()).unwrap().clone();
            if let Some(op) = &self.op {
                match op {
                    TensorOp::Binary(lhs, rhs, kind) => match kind {
                        BinaryOp::Add => {
                            *store.entry(lhs.id()).or_insert(lhs.zeros_like()) += &grad;
                            *store.entry(rhs.id()).or_insert(rhs.zeros_like()) += &grad;
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

        store
    }
}
