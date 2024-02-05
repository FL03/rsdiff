/*
    Appellation: operators <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
use daggy::NodeIndex;

pub struct Operator {
    pub consumers: Vec<NodeIndex>,
    pub inputs: Vec<NodeIndex>,
}

impl Operator {
    pub fn new() -> Self {
        Self {
            consumers: Vec::new(),
            inputs: Vec::new(),
        }
    }
}
