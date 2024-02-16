/*
    Appellation: stack <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/

pub struct Stack<K, V> {
    pub(crate) store: Vec<(K, V)>,
}
