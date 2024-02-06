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
}
