/*
    Appellation: operator <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/

pub trait Operand<Args> {
    type Output;

    fn name(&self) -> &str;

    fn eval(&self, args: Args) -> Self::Output;

    fn grad(&self, args: Args) -> Vec<Self::Output>;
}
