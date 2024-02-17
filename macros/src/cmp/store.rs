/*
    Appellation: store <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
use proc_macro2::TokenStream;
use std::collections::hash_map::Entry;
use std::collections::HashMap;
use syn::Expr;

pub struct GradientStore<K = Expr> {
    pub(crate) store: HashMap<K, TokenStream>,
}

impl<K> GradientStore<K>
where
    K: Eq + std::hash::Hash,
{
    pub fn new() -> Self {
        Self {
            store: HashMap::new(),
        }
    }

    pub fn entry(&mut self, k: K) -> Entry<K, TokenStream> {
        self.store.entry(k)
    }

    pub fn get(&self, k: &K) -> Option<&TokenStream> {
        self.store.get(k)
    }

    pub fn get_mut(&mut self, k: &K) -> Option<&mut TokenStream> {
        self.store.get_mut(k)
    }

    pub fn insert(&mut self, k: K, v: TokenStream) -> Option<TokenStream> {
        self.store.insert(k, v)
    }

    pub fn or_insert(&mut self, k: K, v: TokenStream) -> &mut TokenStream {
        self.entry(k).or_insert(v)
    }

    pub fn remove(&mut self, k: &K) -> Option<TokenStream> {
        self.store.remove(k)
    }

    pub fn retain<F>(&mut self, f: F)
    where
        F: FnMut(&K, &mut TokenStream) -> bool,
    {
        self.store.retain(f);
    }
}

impl GradientStore<Expr> {
    pub fn retain_vars(&mut self) {
        self.retain(|k, _v| matches!(k, Expr::Path(_)));
    }
}

impl<K> std::ops::Deref for GradientStore<K> {
    type Target = HashMap<K, TokenStream>;

    fn deref(&self) -> &Self::Target {
        &self.store
    }
}

impl<K> std::ops::DerefMut for GradientStore<K> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.store
    }
}

impl<K> std::ops::Index<&K> for GradientStore<K>
where
    K: Eq + std::hash::Hash,
{
    type Output = TokenStream;

    fn index(&self, k: &K) -> &Self::Output {
        self.get(k).expect("Key not found")
    }
}

impl<K> std::ops::IndexMut<&K> for GradientStore<K>
where
    K: Eq + std::hash::Hash,
{
    fn index_mut(&mut self, k: &K) -> &mut Self::Output {
        self.get_mut(k).expect("Key not found")
    }
}

impl<K> IntoIterator for GradientStore<K> {
    type Item = (K, TokenStream);
    type IntoIter = std::collections::hash_map::IntoIter<K, TokenStream>;

    fn into_iter(self) -> Self::IntoIter {
        self.store.into_iter()
    }
}

impl<K> FromIterator<(K, TokenStream)> for GradientStore<K>
where
    K: Eq + std::hash::Hash,
{
    fn from_iter<I>(iter: I) -> Self
    where
        I: IntoIterator<Item = (K, TokenStream)>,
    {
        Self {
            store: HashMap::from_iter(iter),
        }
    }
}
