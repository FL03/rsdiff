/*
    Appellation: grad <mod>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
use crate::ops::{BinaryOp, Op,};
use crate::prelude::Scalar;
use crate::tensor::*;
use acme::prelude::AtomicId;

pub(crate) type GradStore<T> = std::collections::BTreeMap<AtomicId, T>;

impl<T> TensorBase<T>
where
    T: Scalar,
{
    pub fn grad(&self) -> GradStore<TensorBase<T>> {
        let mut store = GradStore::new();
        store.insert(self.id().into(), TensorBase::ones_like(self));

        let grad = store.get(&self.id().into()).unwrap().clone();

        if let Some(op) = &self.op {
            match op {
                Op::Unary(_a, kind) => match kind {
                    _ => todo!(),
                },
                Op::Binary(a, b, kind) => match kind {
                    BinaryOp::Add => {
                        *store
                            .entry(a.id().into())
                            .or_insert(TensorBase::zeros_like(a)) += grad.clone();
                        *store
                            .entry(b.id().into())
                            .or_insert(TensorBase::zeros_like(b)) += grad;
                    }
                    _ => todo!(),
                },
                // _ => {}
            }
        }
        store
    }
}
