/*
    Appellation: stores <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
pub use self::gradient::*;

pub(crate) mod gradient;

pub trait Store<K, V> {
    fn get(&self, key: &K) -> Option<&V>;

    fn get_mut(&mut self, key: &K) -> Option<&mut V>;

    fn insert(&mut self, key: K, value: V) -> Option<V>;

    fn remove(&mut self, key: &K) -> Option<V>;
}

// impl<K, V> Store<K, V> for BTreeMap<K, V> where K: Ord {
//     fn get(&self, key: &K) -> Option<&V> {
//         BTreeMap::get(self, &key)
//     }

//     fn get_mut(&mut self, key: &K) -> Option<&mut V> {
//         BTreeMap::get_mut(self, &key)
//     }

//     fn insert(&mut self, key: K, value: V) {
//         BTreeMap::insert(self, key, value);
//     }

//     fn remove(&mut self, key: &K) -> Option<V> {
//         BTreeMap::remove(self, &key)
//     }
// }

// impl<K, V> Store<K, V> for HashMap<K, V> where K: Eq + std::hash::Hash {
//     fn get(&self, key: &K) -> Option<&V> {
//         HashMap::get(self, &key)
//     }

//     fn get_mut(&mut self, key: &K) -> Option<&mut V> {
//         HashMap::get_mut(self, &key)
//     }

//     fn insert(&mut self, key: K, value: V) {
//         HashMap::insert(self, key, value);
//     }

//     fn remove(&mut self, key: &K) -> Option<V> {
//         HashMap::remove(self, &key)
//     }
// }
