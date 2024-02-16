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
use syn::Expr;



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

    pub fn compute_gradients(&self) -> TokenStream {
        let sorted = toposort(&self.graph, None).expect("The graph is cyclic");
        let nodes = sorted.iter().rev().copied().collect::<Vec<_>>();
        // Generate code to compute gradients based on the graph structure
        let mut gradient_code = quote! {};

        // Iterate through the edges of the graph to compute gradients
        for edge in self.graph.raw_edges() {
            // Implement gradient computation logic based on the edge
            // For binary operations, compute gradients using the chain rule
            if let (Some(src), Some(target)) = (
                self.graph.node_weight(edge.source()),
                self.graph.node_weight(edge.target()),
            ) {
                gradient_code = quote! {
                    // Implement gradient computation logic here
                };
            }
        }

        gradient_code
    }

    pub fn build_computational_graph(&mut self, expr: &Expr) {
        let c = self.add_node(expr.clone());

        if let Expr::Binary(expr_binary) = expr {
            // Add edges for left and right children
            let left_id = self.add_node(*expr_binary.left.clone());
            let right_id = self.add_node(*expr_binary.right.clone());
            self.add_edge(left_id, c);
            self.add_edge(right_id, c);

            // Recursive traversal for left and right children
            self.build_computational_graph(&expr_binary.left);
            self.build_computational_graph(&expr_binary.right);
        }
    }
}
