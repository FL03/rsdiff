/*
    Appellation: utils <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
use ndarray::{Array, Data, Dimension, IntoDimension};
use num::Float;
use std::collections::{HashMap, HashSet, VecDeque};
use std::hash::{DefaultHasher, Hash, Hasher};

/// Hashes a dimension using the [DefaultHasher].
pub fn hash_dim<D>(dim: impl IntoDimension<Dim = D>) -> u64
where
    D: Dimension,
{
    let dim = dim.into_dimension();
    let mut s = DefaultHasher::new();
    for i in dim.slice() {
        i.hash(&mut s);
    }
    s.finish()
}

pub fn linarr<A, D>(dim: impl IntoDimension<Dim = D>) -> Array<A, D>
where
    A: Float,
    D: Dimension,
{
    let dim = dim.into_dimension();
    let dview = dim.as_array_view();
    let n = dview.product();
    Array::linspace(A::zero(), A::from(n).unwrap() - A::one(), n)
        .into_shape(dim)
        .expect("linspace err")
}

#[derive(Debug, Eq, PartialEq)]
pub enum TopoligicalSortError {
    CycleDetected,
}

type TopologicalSortResult<Node> = Result<Vec<Node>, TopoligicalSortError>;

use crate::prelude::{TensorExpr, TensorId};
use crate::TensorBase;
use ndarray::{IxDyn, RawData};

pub fn tensor_op_graph<A, S>(tensor: &TensorBase<S, IxDyn>, reverse: bool) -> Vec<&TensorBase<S>>
where
    S: Data<Elem = A>,
{
    // Here, the sorted nodes are passed as an owned value rather than as a mutable reference to workaround some lifetime limitations.
    fn walk<'a, S1>(
        scope: &'a TensorBase<S1>,
        nodes: Vec<&'a TensorBase<S1>>,
        visited: &mut HashMap<TensorId, bool>,
    ) -> (bool, Vec<&'a TensorBase<S1>>)
    where
        S1: RawData,
    {
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
        } else if let Some(op) = scope.op() {
            match op {
                TensorExpr::Binary { lhs, rhs, .. } => {
                    let (tg, nodes) = walk(lhs.as_ref(), nodes, visited);
                    track |= tg;
                    let (tg, nodes) = walk(rhs, nodes, visited);
                    track |= tg;
                    nodes
                }
                TensorExpr::Unary { recv, .. } => {
                    let (tg, nodes) = walk(recv, nodes, visited);
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
    let (_tg, mut nodes) = walk(tensor, Vec::new(), &mut HashMap::new());
    // reverse the nodes; if needed
    if reverse {
        nodes.reverse();
    }
    // return the sorted nodes
    nodes
}

/// Given a directed graph, modeled as a list of edges from source to destination
/// Uses Kahn's algorithm to either:
///     return the topological sort of the graph
///     or detect if there's any cycle
pub fn topological_sort<Node: Hash + Eq + Copy>(
    edges: &Vec<(Node, Node)>,
) -> TopologicalSortResult<Node> {
    // Preparation:
    //  Build a map of edges, organised from source to destinations
    //  Also, count the number of incoming edges by node
    let mut edges_by_source: HashMap<Node, Vec<Node>> = HashMap::default();
    let mut incoming_edges_count: HashMap<Node, usize> = HashMap::default();
    for (source, destination) in edges {
        incoming_edges_count.entry(*source).or_insert(0); // if we haven't seen this node yet, mark it as having 0 incoming nodes
        edges_by_source // add destination to the list of outgoing edges from source
            .entry(*source)
            .or_default()
            .push(*destination);
        // then make destination have one more incoming edge
        *incoming_edges_count.entry(*destination).or_insert(0) += 1;
    }

    // Now Kahn's algorithm:
    // Add nodes that have no incoming edges to a queue
    let mut no_incoming_edges_q = VecDeque::default();
    for (node, count) in &incoming_edges_count {
        if *count == 0 {
            no_incoming_edges_q.push_back(*node);
        }
    }
    // For each node in this "O-incoming-edge-queue"
    let mut sorted = Vec::default();
    while let Some(no_incoming_edges) = no_incoming_edges_q.pop_back() {
        sorted.push(no_incoming_edges); // since the node has no dependency, it can be safely pushed to the sorted result
        incoming_edges_count.remove(&no_incoming_edges);
        // For each node having this one as dependency
        for neighbour in edges_by_source.get(&no_incoming_edges).unwrap_or(&vec![]) {
            if let Some(count) = incoming_edges_count.get_mut(neighbour) {
                *count -= 1; // decrement the count of incoming edges for the dependent node
                if *count == 0 {
                    // `node` was the last node `neighbour` was dependent on
                    incoming_edges_count.remove(neighbour); // let's remove it from the map, so that we can know if we covered the whole graph
                    no_incoming_edges_q.push_front(*neighbour); // it has no incoming edges anymore => push it to the queue
                }
            }
        }
    }
    if incoming_edges_count.is_empty() {
        // we have visited every node
        Ok(sorted)
    } else {
        // some nodes haven't been visited, meaning there's a cycle in the graph
        Err(TopoligicalSortError::CycleDetected)
    }
}
