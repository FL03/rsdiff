/*
    Appellation: storage <mod>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
use std::any::Any;

pub struct Storage {
    pub(crate) data: Vec<Box<dyn Any + 'static>>,
}

impl Storage {
    pub fn new() -> Self {
        Self { data: Vec::new() }
    }

    pub fn push<T: 'static>(&mut self, value: T) {
        self.data.push(Box::new(value));
    }

    pub fn get<T: 'static>(&self, index: usize) -> Option<&T> {
        self.data.get(index).and_then(|value| value.downcast_ref())
    }

    pub fn get_mut<T: 'static>(&mut self, index: usize) -> Option<&mut T> {
        self.data
            .get_mut(index)
            .and_then(|value| value.downcast_mut())
    }
}
