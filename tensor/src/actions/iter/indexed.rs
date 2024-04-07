/*
    Appellation: indexed <mod>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
use crate::prelude::{Layout, Shape, Stride};

pub struct IndexedIter<'a, T: 'a> {
    scope: Option<&'a T>,

}