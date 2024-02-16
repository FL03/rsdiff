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

pub(crate) type ComputeGraph<V = ()> = DiGraph<Expr, V>;

// Function to build a computational graph from the expression
pub fn build_computational_graph(expr: &Expr) -> ComputeGraph {
    // Initialize a graph
    let mut graph = DiGraph::new();

    // Assign IDs to each node in the expression tree
    let mut id_counter = 0;
    assign_node_ids(expr, &mut graph, &mut id_counter);

    // Traverse the expression and add edges to the graph
    traverse_expr(expr, &mut graph);

    graph
}

// Recursive function to assign IDs to each node in the expression tree
fn assign_node_ids(expr: &Expr, graph: &mut ComputeGraph, id_counter: &mut usize) {
    // Assign an ID to the current node
    graph.add_node(expr.clone());
    let current_id = *id_counter;
    *id_counter += 1;

    // Recursive traversal for binary expressions
    if let Expr::Binary(binary_expr) = expr {
        assign_node_ids(&binary_expr.left, graph, id_counter);
        assign_node_ids(&binary_expr.right, graph, id_counter);
    }
}

// Recursive function to traverse the expression and add edges to the graph
fn traverse_expr(expr: &Expr, graph: &mut ComputeGraph) {
    // Recursive traversal for binary expressions
    if let Expr::Binary(binary_expr) = expr {
        // Add edges for left and right children
        let left_id = get_node_id(&binary_expr.left, graph);
        let right_id = get_node_id(&binary_expr.right, graph);
        graph.add_edge(left_id, right_id, ());

        // Recursive traversal for left and right children
        traverse_expr(&binary_expr.left, graph);
        traverse_expr(&binary_expr.right, graph);
    }
}

// Function to get the node ID of an expression node
fn get_node_id(expr: &Expr, graph: &ComputeGraph) -> NodeIndex {
    graph
        .node_indices()
        .filter(|&node| graph[node] == *expr)
        .next()
        .unwrap()
}

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
        // Recursive traversal for binary expressions
        if let Expr::Binary(binary_expr) = expr {
            // Add edges for left and right children
            let left_id = self.add_node(*binary_expr.left.clone());
            let right_id = self.add_node(*binary_expr.right.clone());
            self.add_edge(left_id, c);
            self.add_edge(right_id, c);

            // Recursive traversal for left and right children
            self.build_computational_graph(&binary_expr.left);
            self.build_computational_graph(&binary_expr.right);
        }
    }
}
