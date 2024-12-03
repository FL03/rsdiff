/*
    Appellation: node <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
use rsdiff::id::AtomicId;

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Node<T> {
    pub(crate) id: AtomicId,
    pub(crate) data: T,
    pub(crate) param: bool,
}

impl<T> Node<T> {
    pub fn new(data: T, param: bool) -> Self {
        Self {
            id: AtomicId::new(),
            data,
            param,
        }
    }

    pub fn get_id(&self) -> usize {
        *self.id
    }

    pub fn data(&self) -> &T {
        &self.data
    }

    pub fn data_mut(&mut self) -> &mut T {
        &mut self.data
    }

    pub fn is_param(&self) -> bool {
        self.param
    }

    pub fn set_param(&mut self, param: bool) {
        self.param = param;
    }

    pub fn constant(mut self) -> Self {
        self.param = false;
        self
    }

    pub fn variable(mut self) -> Self {
        self.param = true;
        self
    }
}
