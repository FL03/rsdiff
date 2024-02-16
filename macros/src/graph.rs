/*
    Appellation: graph <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
use petgraph::{
    algo::toposort,
    prelude::{DiGraph, NodeIndex},
};
use proc_macro2::TokenStream;
use quote::quote;
use std::collections::HashMap;
use syn::{Expr, ExprBinary};

pub struct Context {
    graph: DiGraph<Expr, ()>,
}

impl Context {
    pub fn new() -> Self {
        Context {
            graph: DiGraph::new(),
        }
    }

    pub fn add_node(&mut self, expr: Expr) -> NodeIndex {
        self.graph.add_node(expr)
    }

    pub fn add_edge(&mut self, src: NodeIndex, target: NodeIndex) {
        self.graph.add_edge(src, target, ());
    }

    pub fn backward(&self) -> HashMap<NodeIndex, TokenStream> {
        let sorted = toposort(&self.graph, None).expect("The graph is cyclic");
        let target = sorted.last().unwrap().clone();

        let mut stack = Vec::<(NodeIndex, TokenStream)>::new();
        stack.push((target, quote! { 1.0 }));
        let mut store = HashMap::<NodeIndex, TokenStream>::from_iter(stack.clone());

        // Iterate through the edges of the graph to compute gradients
        while let Some((i, grad)) = stack.pop() {
            // get the current node
            let node = &self.graph[i];

            match node {
                Expr::Binary(expr_binary) => {
                    // Compute the gradient of the left child
                    let left = self
                        .graph
                        .neighbors_directed(i, petgraph::Direction::Outgoing)
                        .next()
                        .unwrap();
                    let left_grad = quote! { #grad * #expr_binary.right };
                    stack.push((left, left_grad));

                    // Compute the gradient of the right child
                    let right = self
                        .graph
                        .neighbors_directed(i, petgraph::Direction::Outgoing)
                        .last()
                        .unwrap();
                    let right_grad = quote! { #grad * #expr_binary.left };
                    stack.push((right, right_grad));
                }
                Expr::Unary(expr_unary) => {
                    // Compute the gradient of the child
                    let child = self
                        .graph
                        .neighbors_directed(i, petgraph::Direction::Outgoing)
                        .next()
                        .unwrap();
                    let child_grad = quote! { #grad * #expr_unary.expr };
                    stack.push((child, child_grad));
                }
                _ => {
                    // Do nothing
                }
            }
        }

        store
    }

    pub fn traverse(&mut self, expr: &Expr) {
        let c = self.add_node(expr.clone());

        match expr {
            Expr::Binary(expr_binary) => {
                // Add edges for left and right children
                let a = self.add_node(*expr_binary.left.clone());
                let b = self.add_node(*expr_binary.right.clone());
                self.add_edge(a, c);
                self.add_edge(b, c);

                // Recursive traversal for left and right children
                self.traverse(&expr_binary.left);
                self.traverse(&expr_binary.right);
            }

            Expr::Unary(expr_unary) => {
                // Add an edge for the child
                let a = self.add_node(*expr_unary.expr.clone());
                self.add_edge(a, c);

                // Recursive traversal for the child
                self.traverse(&expr_unary.expr);
            }
            _ => {}
        }
    }
}

fn handle_expr(expr: &Expr) -> Grad {
    match expr {
        Expr::Binary(inner) => handle_binary(inner, quote! { 1.0 }).into(),
        _ => panic!("Unsupported expression!"),
    }
}

fn handle_binary(expr: &ExprBinary, grad: TokenStream) -> BinaryGrad {
    use syn::BinOp;
    let ExprBinary {
        left, op, right, ..
    } = expr.clone();
    match op {
        BinOp::Add(_) => {
            // Implement addition handling
            BinaryGrad {
                left: quote! { #grad },
                right: quote! { #grad },
            }
        }
        BinOp::Mul(_) => {
            // Implement multiplication handling
            BinaryGrad {
                left: quote! { #grad * #right },
                right: quote! { #grad * #left },
            }
        }
        _ => panic!("Unsupported binary operator!"),
    }
}

pub struct BinaryGrad {
    pub left: TokenStream,
    pub right: TokenStream,
}

pub enum Grad {
    Binary(BinaryGrad),
    Unary(TokenStream),
}

impl From<BinaryGrad> for Grad {
    fn from(grad: BinaryGrad) -> Self {
        Grad::Binary(grad)
    }
}
