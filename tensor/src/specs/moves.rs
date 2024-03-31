/*
    Appellation: reshape <mod>
    Contrib: FL03 <jo3mccain@icloud.com>
*/

pub trait Swap {
    type Key;

    fn swap(&mut self, swap: Self::Key, with: Self::Key);
}

impl<T> Swap for [T] {
    type Key = usize;

    fn swap(&mut self, swap: Self::Key, with: Self::Key) {
        self.swap(swap, with);
    }
}
