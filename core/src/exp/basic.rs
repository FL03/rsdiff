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
    values: HashMap<usize, f64>, // Stores values of nodes
}

impl ComputeGraph {
    // Constructor to create a new empty ComputeGraph
    fn new() -> Self {
        ComputeGraph {
            nodes: Vec::new(),
            values: HashMap::new(),
        }
    }

    pub fn variable(&mut self, name: impl ToString, value: Option<f64>) -> usize {
        let id = self.add_node(name.to_string(), vec![], "input".to_string());
        self.values.insert(id, value.unwrap_or(f64::default()));
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
    fn evaluate(&mut self, node_id: usize) -> f64 {
        if let Some(value) = self.values.get(&node_id) {
            return *value;
        }

        let node = self.nodes[node_id].clone();
        let mut result = 0.0;

        // Perform the operation based on the type of node
        match node.operation.as_str() {
            "input" => result = self.values[&node_id], // Placeholder value for input nodes
            "add" => {
                for &input_id in &node.inputs {
                    result += self.evaluate(input_id);
                }
            }
            "multiply" => {
                result = 1.0; // Identity element for multiplication
                for &input_id in &node.inputs {
                    result *= self.evaluate(input_id);
                }
            }
            _ => println!("Unsupported operation"),
        }

        // Store the computed value for future reference
        self.values.insert(node_id, result);
        result
    }

    // Method to compute gradients using backpropagation
    fn grad(&self, target: usize) -> HashMap<usize, f64> {
        let mut gradients: HashMap<usize, f64> = HashMap::new();
        let mut gradients_stack: Vec<(usize, f64)> = Vec::new();

        // Initialize gradient of output node with respect to itself
        gradients.insert(target, 1.0);
        gradients_stack.push((target, 1.0));

        // Compute gradients for all nodes in reverse order
        while let Some((i, grad)) = gradients_stack.pop() {
            let node = &self.nodes[i];

            // Compute gradient for each input of the node
            for &input_id in &node.inputs {
                // Compute gradient contribution from the current node
                let gradient_contribution = match node.operation.as_str() {
                    "add" => grad,
                    "multiply" => {
                        // Compute the product of gradients
                        let output = self.values[&i];
                        let value = self.values[&input_id];
                        grad * output / value
                    }
                    _ => 0.0, // Other operations have zero gradient contribution
                };

                // Update gradient for the input node
                *gradients.entry(input_id).or_insert(0.0) += gradient_contribution;
                gradients_stack.push((input_id, gradient_contribution));
            }
        }

        gradients
    }
}

impl ComputeGraph {
    fn add(&mut self, x: usize, y: usize) -> usize {
        self.add_node("Add".to_string(), vec![x, y], "add".to_string())
    }

    fn multiply(&mut self, x: usize, y: usize) -> usize {
        self.add_node("Multiply".to_string(), vec![x, y], "multiply".to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // #[ignore = "The basic compute graph fails to compute the gradients correctly."]
    #[test]
    fn test_basic_graph() {
        // Create a new computational graph
        let mut graph = ComputeGraph::new();

        // Add nodes to the graph
        let x = graph.variable("x".to_string(), Some(1.0));
        let y = graph.variable("y".to_string(), Some(2.0));
        let c = graph.add(x, y);
        let d = graph.multiply(c, y);

        // Evaluate nodes
        let res = graph.evaluate(c);
        assert_eq!(res, 3.0);
        let res = graph.evaluate(d);
        assert_eq!(res, 6.0);

        // Compute gradients
        let gc = graph.grad(c);
        assert_eq!(gc[&x], 1.0);
        assert_eq!(gc[&y], 1.0);

        let gd = graph.grad(d);

        // Check gradients
        assert_eq!(gd[&x], 2.0);
        assert_eq!(gd[&y], 5.0);
    }
}
