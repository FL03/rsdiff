/*
    Appellation: grad <mod>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
use crate::actions::grad::GradStore;
use crate::prelude::{Scalar, TensorExpr, TensorId, TensorResult};
use crate::TensorBase;
use acme::prelude::{BinaryOp, Store, UnaryOp};

pub(crate) type Visited<K = TensorId> = std::collections::HashMap<K, bool>;

macro_rules! entry {
    ($ctx:expr, $entry:expr) => {
        entry!($ctx, $entry, $entry.zeros_like())
    };
    ($ctx:expr, $entry:expr, $default:expr) => {
        $ctx.entry($entry.id()).or_insert($default)
    };
}

impl<T> TensorBase<T>
where
    T: Scalar,
{
    /// [toposort](TensorBase::toposort) is a utilitarian functions that returns a topologically sorted list of nodes.
    fn toposort(&self, reverse: bool) -> Vec<&TensorBase<T>> {
        // Here, the sorted nodes are passed as an owned value rather than as a mutable reference to workaround some lifetime limitations.
        fn walk<'a, T>(
            scope: &'a TensorBase<T>,
            nodes: Vec<&'a TensorBase<T>>,
            visited: &mut Visited<TensorId>,
        ) -> (bool, Vec<&'a TensorBase<T>>) {
            if let Some(&tg) = visited.get(&scope.id()) {
                return (tg, nodes);
            }
            // track the gradient of the current node
            let mut track = false;
            // recursively call on the children nodes
            let mut nodes = if scope.is_variable() {
                // Do not call recursively on the "leaf" nodes.
                track = true;
                nodes
            } else if let Some(op) = scope.op().op() {
                match op {
                    TensorExpr::Binary(lhs, rhs, _kind) => {
                        let (tg, nodes) = walk(lhs, nodes, visited);
                        track |= tg;
                        let (tg, nodes) = walk(rhs, nodes, visited);
                        track |= tg;
                        nodes
                    }
                    TensorExpr::Unary(a, _kind) => {
                        let (tg, nodes) = walk(a, nodes, visited);
                        track |= tg;
                        nodes
                    }
                    _ => nodes,
                }
            } else {
                nodes
            };
            visited.insert(scope.id(), track);
            if track {
                nodes.push(scope);
            }
            (track, nodes)
        }
        // walk through the dag
        let (_tg, mut nodes) = walk(self, Vec::new(), &mut Visited::new());
        // reverse the nodes; if needed
        if reverse {
            nodes.reverse();
        }
        // return the sorted nodes
        nodes
    }

    pub fn grad(&self) -> TensorResult<GradStore<T>> {
        // get the sorted nodes
        let sorted = self.toposort(true);
        // initialize a new gradient store
        let mut store = GradStore::new();
        // insert the gradient w.r.t. the current node
        store.insert(self.id(), self.ones_like());

        for node in sorted.iter() {
            if node.is_variable() {
                continue;
            }
            // get the gradient of the node
            let grad = store.remove(&node.id()).expect("Gradient not found");
            // detach the gradient
            let grad = grad.detach();
            // handle the different types of operations
            if let Some(op) = &*node.op {
                match op {
                    TensorExpr::Binary(lhs, rhs, kind) => match kind {
                        BinaryOp::Add(_) => {
                            *entry!(store, lhs) += &grad;
                            *entry!(store, rhs) += &grad;
                        }
                        BinaryOp::Div(_) => {
                            *entry!(store, lhs) += &grad / rhs.as_ref();
                            *entry!(store, rhs) -=
                                &grad * lhs.as_ref() / (rhs.as_ref() * rhs.as_ref());
                        }
                        BinaryOp::Mul(_) => {
                            *entry!(store, lhs) += &grad * rhs.as_ref();
                            *entry!(store, rhs) += &grad * lhs.as_ref();
                        }
                        BinaryOp::Sub(_) => {
                            *entry!(store, lhs) += &grad;
                            *entry!(store, rhs) -= &grad;
                        }
                        _ => todo!(),
                    },
                    TensorExpr::BinaryScalar(lhs, rhs, kind) => match kind {
                        BinaryOp::Add(_) => {
                            *entry!(store, lhs) += &grad;
                        }
                        BinaryOp::Div(_) => {
                            *entry!(store, lhs) += &grad / *rhs;
                        }
                        BinaryOp::Mul(_) => {
                            *entry!(store, lhs) += &grad * *rhs;
                        }
                        BinaryOp::Pow => {
                            *entry!(store, lhs) += &grad * *rhs * lhs.pow(*rhs - T::one());
                        }
                        BinaryOp::Sub(_) => {
                            *entry!(store, lhs) += &grad;
                        }
                        _ => todo!(),
                    },
                    TensorExpr::Unary(val, kind) => match kind {
                        UnaryOp::Cos => {
                            *entry!(store, val) -= &grad * val.sin();
                        }
                        UnaryOp::Cosh => {
                            *entry!(store, val) += &grad * val.sinh();
                        }
                        UnaryOp::Exp => {
                            *entry!(store, val) += &grad * val.exp();
                        }
                        UnaryOp::Neg => {
                            *entry!(store, val) -= &grad;
                        }
                        UnaryOp::Sin => {
                            *entry!(store, val) += &grad * val.cos();
                        }
                        UnaryOp::Sinh => {
                            *entry!(store, val) += &grad * val.cosh();
                        }
                        UnaryOp::Sqrt => {
                            *entry!(store, val) +=
                                &grad / (val.clone().sqrt() * T::from(2).unwrap());
                        }
                        UnaryOp::Tan => {
                            *entry!(store, val) += &grad / val.clone().cos().sqr();
                        }

                        _ => todo!(),
                    },
                    _ => {}
                }
            }
        }

        Ok(store)
    }
}
