/*
    Appellation: node <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
//! # Node
//!
//! A computational graph relies on weighted nodes to represent constants, operations, and variables.
//! The edges connecting to any given node are considered to be inputs and help to determine the flow of information
use super::{Config, GradientUpdater};
use crate::cmp::id::Id;

pub struct Node<C: Config> {
    inputs: Vec<Option<Id>>, // Edges denote which nodes are connected to the current node, thus serving as inputs
    updater: Option<GradientUpdater<C>>,
}

impl<C> Node<C>
where
    C: Config,
{
    pub fn new(
        inputs: impl IntoIterator<Item = Option<Id>>,
        updater: Option<GradientUpdater<C>>,
    ) -> Self {
        Self {
            inputs: Vec::from_iter(inputs),
            updater,
        }
    }

    pub fn clear(&mut self) {
        self.inputs.clear();
        self.updater = None;
    }

    pub fn inputs(&self) -> &Vec<Option<Id>> {
        &self.inputs
    }

    pub fn inputs_mut(&mut self) -> &mut Vec<Option<Id>> {
        &mut self.inputs
    }

    pub fn updater(&self) -> Option<&GradientUpdater<C>> {
        self.updater.as_ref()
    }
}
