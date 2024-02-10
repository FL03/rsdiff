/*
    Appellation: operators <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
use daggy::NodeIndex;

pub struct Operator {
    inputs: Vec<NodeIndex>,
    name: String,
}

impl Operator {
    pub fn new() -> Self {
        Self {
            inputs: Vec::new(),
            name: String::new(),
        }
    }

    pub fn with_name(mut self, name: impl ToString) -> Self {
        self.name = name.to_string();
        self
    }

    pub fn inputs(&self) -> &[NodeIndex] {
        &self.inputs
    }

    pub fn name(&self) -> &str {
        &self.name
    }
}
