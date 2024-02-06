/*
    Appellation: tensor <mod>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
use crate::core::cmp::id::AtomicId;
use crate::store::Layout;

pub struct Tensor {
    id: AtomicId,
    layout: Layout,
}
