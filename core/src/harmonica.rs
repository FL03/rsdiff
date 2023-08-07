/*
    Appellation: harmonica <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... summary ...
*/

pub type HarmonicInterpolation<S, T> = dyn Fn(S) -> T;

pub trait Actionable<S> {
    type Output;

    fn dirac(&self) -> &HarmonicInterpolation<S, Self::Output>;
    fn data(&self) -> S;
    fn transition(&self) -> Self::Output {
        self.dirac()(self.data())
    }
}