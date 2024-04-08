/*
    Appellation: position <mod>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
use crate::prelude::{Layout, Shape, Stride};

/// An iterator over the positions of an n-dimensional tensor
///
pub struct PositionIter {
    next: Option<usize>,
    position: Vec<usize>,
    shape: Shape,
    stride: Stride,
}

impl PositionIter {
    pub fn new(offset: usize, shape: Shape, stride: Stride) -> Self {
        let elems: usize = shape.size();
        let next = if elems == 0 {
            None
        } else {
            // This applies to the scalar case.
            Some(offset)
        };
        Self {
            next,
            position: vec![0; *shape.rank()],
            shape,
            stride,
        }
    }

    pub(crate) fn index(&self, index: impl AsRef<[usize]>) -> usize {
        index
            .as_ref()
            .iter()
            .zip(self.stride.iter())
            .map(|(i, s)| i * s)
            .sum()
    }
}

impl DoubleEndedIterator for PositionIter {
    fn next_back(&mut self) -> Option<Self::Item> {
        let (pos, _idx) = if let Some(item) = self.next() {
            item
        } else {
            return None;
        };
        let position = self
            .shape
            .iter()
            .zip(pos.iter())
            .map(|(s, p)| s - p)
            .collect();
        let scope = self.index(&position);
        Some((position, scope))
    }
}

impl Iterator for PositionIter {
    type Item = (Vec<usize>, usize);

    fn next(&mut self) -> Option<Self::Item> {
        let scope = match self.next {
            None => return None,
            Some(storage_index) => storage_index,
        };
        let mut updated = false;
        let mut next = scope;
        for ((multi_i, max_i), stride_i) in self
            .position
            .iter_mut()
            .zip(self.shape.iter())
            .zip(self.stride.iter())
            .rev()
        {
            let next_i = *multi_i + 1;
            if next_i < *max_i {
                *multi_i = next_i;
                updated = true;
                next += stride_i;
                break;
            } else {
                next -= *multi_i * stride_i;
                *multi_i = 0
            }
        }
        self.next = if updated { Some(next) } else { None };
        Some((self.position.clone(), scope))
    }
}

impl From<Layout> for PositionIter {
    fn from(layout: Layout) -> Self {
        let Layout {
            offset,
            shape,
            strides,
        } = layout;

        Self::new(offset, shape, strides)
    }
}

impl<'a> From<&'a Layout> for PositionIter {
    fn from(layout: &'a Layout) -> Self {
        Self::from(layout.clone())
    }
}
