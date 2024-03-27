/*
    Appellation: dtype <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/

use std::any::TypeId;

pub enum DType {
    Float(Float),
    Integer(Integer),
}

pub enum Float {
    F32,
    F64,
}

impl Float {
    pub fn from_type<T>(_value: &T) -> Result<Self, ()>
    where
        T: 'static,
    {
        if TypeId::of::<T>() == TypeId::of::<f32>() {
            Ok(Float::F32)
        } else if TypeId::of::<T>() == TypeId::of::<f64>() {
            Ok(Float::F64)
        } else {
            Err(())
        }
    }
}

impl From<f32> for Float {
    fn from(_: f32) -> Self {
        Float::F32
    }
}

impl From<f64> for Float {
    fn from(_: f64) -> Self {
        Float::F64
    }
}

pub struct Integer {
    pub bits: NumBits,
    pub signed: bool,
}

#[repr(u8)]
pub enum NumBits {
    B8 = 8,
    B16 = 16,
    B32 = 32,
    B64 = 64,
    B128 = 128,
    BSize,
}
