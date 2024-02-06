/*
    Appellation: graphs <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
//! # Graphs
//!
//!
pub use self::{edge::*, graph::*, node::*};

pub(crate) mod edge;
pub(crate) mod graph;
pub(crate) mod node;

pub mod basic;
pub mod ops;

pub use daggy::NodeIndex as Id;

use crate::prelude::{Evaluate, Gradient, GradientStore, Result, Variable};
use daggy::NodeIndex;
use std::marker::PhantomData;
use std::sync::Arc;

pub type GradientUpdater<C> = Arc<
    dyn Fn(&mut <C as Config>::Grad, &mut <C as Config>::Store, NodeIndex) -> Result<()>
        + Send
        + Sync,
>;

pub trait Config: Default {
    type DType;
    type Eval: Clone + Default;
    type Grad;
    type Store;
}



// pub struct Backend

pub struct ComputeConfig<T, E> {
    dtype: PhantomData<T>,
    eval: PhantomData<E>,
}

impl<T, E> Default for ComputeConfig<T, E> {
    fn default() -> Self {
        Self {
            dtype: PhantomData,
            eval: PhantomData,
        }
    }
}

impl<T, E> Config for ComputeConfig<T, E>
where
    E: Clone + Default,
{
    type DType = T;
    type Eval = E;
    type Grad = E;
    type Store = GradientStore<usize>;
}

#[cfg(test)]
mod tests {

}
