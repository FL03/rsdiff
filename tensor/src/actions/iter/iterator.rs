/*
    Appellation: iterator <mod>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
use crate::prelude::Order;

pub struct Iter {
    order: Order,
}

impl Iter {
    pub fn new(order: Order) -> Self {
        Self { order }
    }

    pub fn order(&self) -> Order {
        self.order
    }
}
