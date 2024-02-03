/*
    Appellation: stores <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
pub use self::gradient::*;

pub(crate) mod gradient;

use crate::graphs::Arithmetic;

pub trait Store<K, V> {
    fn get(&self, key: &K) -> Option<&V>;

    fn get_mut(&mut self, key: &K) -> Option<&mut V>;

    fn insert(&mut self, key: K, value: V);

    fn remove(&mut self, key: &K) -> Option<V>;

    fn add_gradient<G>(&mut self, graph: &mut G, id: K, value: &V)
    where
        G: Arithmetic<V>,
        V: Clone + 'static,
    {
        match self.get_mut(&id) {
            Some(gradient) => {
                *gradient = graph.add(gradient.clone(), value.clone());
            }
            None => {
                self.insert(id, value.clone());
            }
        }
    }
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
