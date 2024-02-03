/*
    Appellation: node <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
use crate::ids::Id;

pub struct Node {
    inputs: Vec<Option<Id>>,
}

impl Node {
    pub fn new(inputs: impl IntoIterator<Item = Option<Id>>) -> Self {
        Self {
            inputs: Vec::from_iter(inputs),
        }
    }

    pub fn inputs(&self) -> &Vec<Option<Id>> {
        &self.inputs
    }

    pub fn inputs_mut(&mut self) -> &mut Vec<Option<Id>> {
        &mut self.inputs
    }
}
