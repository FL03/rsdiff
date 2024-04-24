/*
    Appellation: graph <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
use super::edge::Edge;
use super::node::Node;
use super::DynamicGraph;
use crate::prelude::GraphResult as Result;
use crate::NodeIndex;

use acme::ops::{Arithmetic, BinaryOp, Op, UnaryOp};
use acme::prelude::Scalar;
use core::ops::Index;
use petgraph::algo::toposort;
use std::collections::HashMap;

macro_rules! entry {
    ($ctx:ident[$key:expr]) => {
        entry!(@base $ctx[$key]).or_default()
    };
    ($ctx:ident[$key:expr], $val:expr) => {
        entry!(@base $ctx[$key].or_insert($val))
    };
    (@base $ctx:ident[$key:expr].$call:ident($val:expr)) => {
        entry!($ctx[$key]).$call:ident($val)
    };
    (@base $ctx:ident[$key:expr]) => {
        $ctx.entry($key)
    };

}

macro_rules! push {
    ($ctx:expr, $(($key:expr, $val:expr)),*) => {
        $(push!(@impl $ctx, $key, $val);)*
    };

    ($ctx:expr, $key:expr, $val:expr) => {
        push!(@impl $ctx, $key, $val)
    };
    (@impl $ctx:expr, $key:expr, $val:expr) => {
        $ctx.push(($key, $val))
    };

}

macro_rules! binop {
    ($($call:ident),*) => {
        $(binop!(@impl $call);)*
    };
    (@impl $call:ident) => {
        pub fn $call(&mut self, lhs: NodeIndex, rhs: NodeIndex) -> NodeIndex {
            self.binary(lhs, rhs, BinaryOp::$call())
        }
    };
}

macro_rules! unop {
    ($($call:ident),*) => {
        $(unop!(@impl $call);)*
    };
    (@impl $call:ident) => {
        pub fn $call(&mut self, recv: NodeIndex) -> NodeIndex {
            self.unary(recv, UnaryOp::$call())
        }
    };
}

pub struct Dcg<T> {
    store: DynamicGraph<T>,
}

impl<T> Default for Dcg<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> Dcg<T> {
    pub fn new() -> Self {
        Dcg {
            store: DynamicGraph::new(),
        }
    }

    pub fn binary(&mut self, lhs: NodeIndex, rhs: NodeIndex, op: impl Into<BinaryOp>) -> NodeIndex {
        let c = self.store.add_node(Node::binary(lhs, rhs, op));
        self.store.add_edge(lhs, c, Edge::new([rhs], lhs));
        self.store.add_edge(rhs, c, Edge::new([lhs], rhs));
        c
    }

    pub fn constant(&mut self, value: T) -> NodeIndex {
        self.input(false, value)
    }

    pub fn get(&self, index: NodeIndex) -> Option<&Node<T>> {
        self.store.node_weight(index)
    }

    pub fn include(&mut self, node: impl Into<Node<T>>) -> NodeIndex {
        self.store.add_node(node.into())
    }

    pub fn input(&mut self, param: bool, value: T) -> NodeIndex {
        self.store.add_node(Node::input(param, value))
    }

    pub fn op(
        &mut self,
        inputs: impl IntoIterator<Item = NodeIndex>,
        op: impl Into<Op>,
    ) -> NodeIndex {
        let args = Vec::from_iter(inputs);

        let c = self.store.add_node(Node::op(args.clone(), op));
        for arg in args.iter() {
            self.store.add_edge(*arg, c, Edge::new(args.clone(), *arg));
        }
        c
    }

    pub fn remove(&mut self, index: NodeIndex) -> Option<Node<T>> {
        self.store.remove_node(index)
    }

    pub fn unary(&mut self, input: NodeIndex, op: impl Into<UnaryOp>) -> NodeIndex {
        let c = self.store.add_node(Node::unary(input, op));
        self.store.add_edge(input, c, Edge::new([input], input));
        c
    }

    pub fn variable(&mut self, value: T) -> NodeIndex {
        self.input(true, value)
    }

    binop!(add, div, mul, pow, rem, sub);

    unop!(
        abs, acos, acosh, asin, asinh, atan, atanh, ceil, cos, cosh, exp, floor, inv, ln, neg,
        recip, sin, sinh, sqr, sqrt, tan, tanh
    );
}

impl<T> Dcg<T>
where
    T: Scalar<Real = T>,
{
    pub fn backward(&self) -> Result<HashMap<NodeIndex, T>> {
        let sorted = toposort(&self.store, None)?;
        let target = sorted.last().unwrap();
        self.gradient(*target)
    }
    pub fn gradient(&self, target: NodeIndex) -> Result<HashMap<NodeIndex, T>> {
        let mut store = HashMap::<NodeIndex, T>::new();
        // initialize the stack
        let mut stack = Vec::<(NodeIndex, T)>::new();
        // start by computing the gradient of the target w.r.t. itself
        stack.push((target, T::one()));
        store.insert(target, T::one());

        while let Some((i, grad)) = stack.pop() {
            let node = &self[i];

            match node {
                Node::Binary { lhs, rhs, op } => match op {
                    BinaryOp::Arith(inner) => match inner {
                        Arithmetic::Add(_) => {
                            *entry!(store[*lhs]) += grad;
                            *entry!(store[*rhs]) += grad;

                            push!(stack, (*lhs, grad), (*rhs, grad));
                        }
                        Arithmetic::Div(_) => {
                            let lhs_grad = grad / self[*rhs].value();
                            let rhs_grad = grad * self[*lhs].value() / self[*rhs].value().powi(2);
                            *entry!(store[*lhs]) += lhs_grad;
                            *entry!(store[*rhs]) += rhs_grad;

                            push!(stack, (*lhs, lhs_grad), (*rhs, rhs_grad));
                        }
                        Arithmetic::Mul(_) => {
                            let lhs_grad = grad * self[*rhs].value();
                            let rhs_grad = grad * self[*lhs].value();
                            *entry!(store[*lhs]) += lhs_grad;
                            *entry!(store[*rhs]) += rhs_grad;
                            push!(stack, (*lhs, lhs_grad), (*rhs, rhs_grad));
                        }
                        Arithmetic::Pow(_) => {
                            let lhs_grad = grad
                                * self[*rhs].value()
                                * self[*lhs].value().powf(self[*rhs].value() - T::one());
                            let rhs_grad = grad
                                * self[*lhs].value().powf(self[*rhs].value())
                                * (self[*lhs].value().ln());
                            *entry!(store[*lhs]) += lhs_grad;
                            *entry!(store[*rhs]) += rhs_grad;

                            push!(stack, (*lhs, lhs_grad), (*rhs, rhs_grad));
                        }
                        Arithmetic::Sub(_) => {
                            *entry!(store[*lhs]) += grad;
                            *entry!(store[*rhs]) -= grad;

                            push!(stack, (*lhs, grad), (*rhs, -grad));
                        }
                        _ => todo!(),
                    },
                    _ => todo!(),
                },
                Node::Unary { .. } => {
                    unimplemented!();
                }
                Node::Input { param, .. } => {
                    if *param {
                        continue;
                    }
                    *store.entry(i).or_default() += grad;
                    stack.push((i, grad));
                }
                _ => {}
            }
        }

        Ok(store)
    }
}

impl<T> Index<NodeIndex> for Dcg<T> {
    type Output = Node<T>;

    fn index(&self, index: NodeIndex) -> &Self::Output {
        self.get(index).unwrap()
    }
}
