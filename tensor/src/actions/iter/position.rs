/*
    Appellation: position <mod>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
use crate::shape::{Layout, Shape, Stride};

pub struct Position {
    pub(crate) index: usize,
    pub(crate) position: Vec<usize>,
}

/// An iterator over the positions of an n-dimensional tensor.
pub struct LayoutIter {
    layout: Layout,
    next: Option<usize>,
    pos: Vec<usize>,
}

impl LayoutIter {
    pub fn new(layout: Layout) -> Self {
        let next = if layout.size() == 0 {
            None
        } else {
            // This applies to the scalar case.
            Some(layout.offset())
        };
        let pos = vec![0; *layout.rank()];
        Self { next, layout, pos }
    }

    pub unsafe fn from_parts(offset: usize, shape: Shape, strides: Stride) -> Self {
        let layout = Layout::new(offset, shape, strides);
        Self::new(layout)
    }

    pub(crate) fn index(&self, index: impl AsRef<[usize]>) -> usize {
        self.layout.index(index)
    }
}

impl DoubleEndedIterator for LayoutIter {
    fn next_back(&mut self) -> Option<Self::Item> {
        let Position { position, .. } = if let Some(item) = self.next() {
            item
        } else {
            return None;
        };
        let rev = self
            .layout
            .shape()
            .get_final_position()
            .iter()
            .zip(position.iter())
            .map(|(s, p)| (s - p))
            .collect();
        let pos = Position::new(self.index(&rev), rev);
        Some(pos)
    }
}

impl ExactSizeIterator for LayoutIter {
    fn len(&self) -> usize {
        self.layout.size()
    }
}

impl Iterator for LayoutIter {
    type Item = Position;

    fn next(&mut self) -> Option<Self::Item> {
        let index = match self.next {
            None => return None,
            Some(i) => i,
        };
        let cur = Position::new(index, self.pos.clone());
        let mut updated = false;
        let mut next = index;
        for ((i, j), s) in self
            .pos
            .iter_mut()
            .zip(self.layout.shape.iter())
            .zip(self.layout.strides.iter())
            .rev()
        {
            let next_i = *i + 1;
            if next_i < *j {
                *i = next_i;
                updated = true;
                next += s;
                break;
            } else {
                next -= *i * s;
                *i = 0;
            }
        }
        self.next = if updated { Some(next) } else { None };
        Some(cur)
    }
}

mod impl_position {
    use super::Position;

    impl Position {
        pub fn new(index: usize, position: Vec<usize>) -> Self {
            Self { index, position }
        }

        pub fn first(rank: usize) -> Self {
            Self::new(0, vec![0; rank])
        }

        pub fn index(&self) -> usize {
            self.index
        }

        pub fn position(&self) -> Vec<usize> {
            self.position.clone()
        }
    }

    impl From<(usize, Vec<usize>)> for Position {
        fn from((idx, pos): (usize, Vec<usize>)) -> Self {
            Self::new(idx, pos)
        }
    }

    impl From<(Vec<usize>, usize)> for Position {
        fn from((pos, idx): (Vec<usize>, usize)) -> Self {
            Self::new(idx, pos)
        }
    }

    impl From<Position> for (usize, Vec<usize>) {
        fn from(pos: Position) -> (usize, Vec<usize>) {
            (pos.index, pos.position)
        }
    }
}
