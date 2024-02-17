/*
    Appellation: graph <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
use petgraph::{
    algo::toposort,
    prelude::{DiGraph, NodeIndex},
};
use proc_macro2::TokenStream;
use quote::{quote, ToTokens};
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
            Expr::Binary(inner) => {
                let ExprBinary { left, right, .. } = inner;
                // Add edges for left and right children
                let a = self.add_node(*left.clone());
                let b = self.add_node(*right.clone());
                self.add_edge(a, c);
                self.add_edge(b, c);

                // Recursive traversal for left and right children
                self.traverse(left);
                self.traverse(right);
            }

            Expr::Unary(inner) => {
                // Add an edge for the child
                let a = self.add_node(*inner.expr.clone());
                self.add_edge(a, c);

                // Recursive traversal for the child
                self.traverse(&inner.expr);
            }
            _ => {}
        }
    }
}

fn handle_expr(expr: &Expr) -> Grad {
    match expr {
        Expr::Binary(inner) => handle_binary(inner).into(),
        _ => panic!("Unsupported expression!"),
    }
}

fn handle_binary(expr: &ExprBinary) -> BinaryGrad {
    use syn::BinOp;
    let ExprBinary {
        left, op, right, ..
    } = expr.clone();

    let dl = handle_expr(&left);
    let dr = handle_expr(&right);
    match op {
        BinOp::Add(_) => {
            // Implement addition handling
            BinaryGrad {
                left: quote! { #dl },
                right: quote! { #dr },
            }
        }
        BinOp::Mul(_) => {
            // Implement multiplication handling
            BinaryGrad {
                left: quote! { #dl * #right },
                right: quote! { #dr * #left },
            }
        }
        _ => panic!("Unsupported binary operator!"),
    }
}

pub struct BinaryGrad {
    pub left: TokenStream,
    pub right: TokenStream,
}

impl ToTokens for BinaryGrad {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        self.left.to_tokens(tokens);
        self.right.to_tokens(tokens);
    }
}

pub enum Grad {
    Binary(BinaryGrad),
    Unary(TokenStream),
    Verbatim(TokenStream),
}

impl From<BinaryGrad> for Grad {
    fn from(grad: BinaryGrad) -> Self {
        Grad::Binary(grad)
    }
}

impl From<TokenStream> for Grad {
    fn from(grad: TokenStream) -> Self {
        Grad::Verbatim(grad)
    }
}

impl ToTokens for Grad {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        match self {
            Grad::Binary(grad) => {
                grad.to_tokens(tokens);
            }
            Grad::Unary(grad) => {
                grad.to_tokens(tokens);
            }
            Grad::Verbatim(grad) => {
                grad.to_tokens(tokens);
            }
        }
    }
}
