/*
    Appellation: traits <mod>
    Contrib: FL03 <jo3mccain@icloud.com>
*/

pub trait Group {}

pub(crate) mod prelude {
    pub use super::Group;
}

#[cfg(test)]
mod tests {

}
