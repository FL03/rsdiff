/*
    Appellation: gradient <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/

pub trait Gradient<T> {
    type Gradient;

    fn grad(&self, args: T) -> Self::Gradient;
}

pub struct Derivative<T> {
    pub wrt: T,
    pub f: Box<dyn Fn(T) -> T>,
}
