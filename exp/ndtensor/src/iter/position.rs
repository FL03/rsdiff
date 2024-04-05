/*
    Appellation: position <mod>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
use acme::prelude::{Layout, Shape, Stride};

///
pub struct IndexIter<'a> {
    next: Option<usize>,
    position: Vec<usize>,
    shape: &'a Shape,
    stride: &'a Stride,
}

impl<'a> IndexIter<'a> {
    pub fn new(offset: usize, shape: &'a Shape, stride: &'a Stride) -> Self {
        let elem_count: usize = shape.iter().product();
        let next = if elem_count == 0 {
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

impl<'a> DoubleEndedIterator for IndexIter<'a> {
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
        println!("{:?}", &position);
        Some((position, scope))
        // unimplemented!()
    }
}

impl<'a> Iterator for IndexIter<'a> {
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

impl<'a> From<&'a Layout> for IndexIter<'a> {
    fn from(layout: &'a Layout) -> Self {
        Self::new(layout.offset(), layout.shape(), layout.strides())
    }
}
