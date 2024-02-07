/*
    Appellation: storage <mod>
    Contrib: FL03 <jo3mccain@icloud.com>
*/

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Storage<T> {
    pub(crate) data: Vec<T>,
}

impl<T> Storage<T> {
    pub fn new() -> Self {
        Self { data: Vec::new() }
    }

    pub fn insert(&mut self, index: usize, value: T) {
        self.data.insert(index, value);
    }

    pub fn push(&mut self, value: T) {
        self.data.push(value);
    }

    pub fn get(&self, index: usize) -> Option<&T> {
        self.data.get(index)
    }

    pub fn get_mut(&mut self, index: usize) -> Option<&mut T> {
        self.data.get_mut(index)
    }

    pub fn remove(&mut self, index: usize) -> T {
        self.data.remove(index)
    }
}

impl<T> Default for Storage<T> {
    fn default() -> Self {
        Self::new()
    }
}
