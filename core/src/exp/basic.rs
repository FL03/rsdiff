/*
    Appellation: basic <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
use std::collections::HashMap;

// Node structure representing a node in the computational graph
#[derive(Clone, Debug)]
struct Node {
    id: usize,
    name: String,
    inputs: Vec<usize>, // Stores indices of parent nodes
    operation: String,  // Operation performed by the node
}

// Computational graph structure
#[derive(Clone, Debug)]
struct ComputeGraph {
    nodes: Vec<Node>,                 // Stores all nodes in the graph
    node_values: HashMap<usize, f64>, // Stores values of nodes
    gradients: HashMap<usize, f64>,   // Stores gradients of nodes
}

impl ComputeGraph {
    // Constructor to create a new empty ComputeGraph
    fn new() -> Self {
        ComputeGraph {
            nodes: Vec::new(),
            node_values: HashMap::new(),
            gradients: HashMap::new(),
        }
    }

    pub fn variable(&mut self, name: impl ToString, value: Option<f64>) -> usize {
        let id = self.add_node(name.to_string(), vec![], "input".to_string());
        self.node_values.insert(id, value.unwrap_or(f64::default()));
        id
    }

    // Method to add a node to the graph
    fn add_node(
        &mut self,
        name: impl ToString,
        inputs: Vec<usize>,
        operation: impl ToString,
    ) -> usize {
        let id = self.nodes.len();
        let node = Node {
            id,
            name: name.to_string(),
            inputs,
            operation: operation.to_string(),
        };
        self.nodes.push(node);
        id
    }

    // Method to evaluate the value of a node recursively
    fn evaluate_node(&mut self, node_id: usize) -> f64 {
        if let Some(value) = self.node_values.get(&node_id) {
            return *value;
        }

        let node = self.nodes[node_id].clone();
        let mut result = 0.0;

        // Perform the operation based on the type of node
        match node.operation.as_str() {
            "input" => result = self.node_values[&node_id], // Placeholder value for input nodes
            "add" => {
                for &input_id in &node.inputs {
                    result += self.evaluate_node(input_id);
                }
            }
            "multiply" => {
                result = 1.0; // Identity element for multiplication
                for &input_id in &node.inputs {
                    result *= self.evaluate_node(input_id);
                }
            }
            _ => println!("Unsupported operation"),
        }

        // Store the computed value for future reference
        self.node_values.insert(node_id, result);
        result
    }

    // Method to compute gradients using backpropagation
    fn grad(&mut self, target: usize) {
        // Initialize gradient of output node with respect to itself
        self.gradients.insert(target, 1.0);

        // Compute gradients for all nodes in reverse order
        for i in self.clone().nodes.iter().rev().map(|node| node.id) {
            let node = self.nodes[i].clone();
            let grad = *self.gradients.get(&node.id).unwrap_or(&Default::default());

            // Compute gradient for each input of the node
            for &input_id in &node.inputs {
                let mut di = 0.0;
                // Compute gradient contribution from the current node
                let gradient_contribution = match node.operation.as_str() {
                    "add" => {
                        di += grad;
                        di
                    },
                    "multiply" => {
                        
                        // Compute the product of gradients
                        // let output_value = self.node_values[&node.id];
                        let value = self.node_values[&input_id];
                        di += grad * value;
                        di
                    }
                    _ => 0.0, // Other operations have zero gradient contribution
                };

                // Update gradient for the input node
                *self.gradients.entry(input_id).or_insert(0.0) += di;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_graph() {
        // Create a new computational graph
        let mut graph = ComputeGraph::new();

        // Add nodes to the graph
        let x = graph.variable("x".to_string(), Some(1.0));
        let y = graph.variable("y".to_string(), Some(2.0));
        let add_node = graph.add_node("Add".to_string(), vec![x, y], "add".to_string());
        let multiply_node = graph.add_node(
            "Multiply".to_string(),
            vec![add_node, y],
            "multiply".to_string(),
        );

        // Evaluate nodes
        let res = graph.evaluate_node(add_node);
        assert_eq!(res, 3.0);
        let res = graph.evaluate_node(multiply_node);
        assert_eq!(res, 6.0);

        graph.grad(add_node);
        graph.grad(multiply_node);

        // Check gradients
        assert_eq!(graph.gradients[&x], 2.0);
        assert_eq!(graph.gradients[&y], 5.0);
    }
}
