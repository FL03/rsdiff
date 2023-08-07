/*
    Appellation: conduit <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... summary ...
*/
pub trait Conduit<T> {
    type Sender;
    type Receiver;

    fn sender(&self) -> Self::Sender;
    fn receiver(&self) -> Self::Receiver;
}
