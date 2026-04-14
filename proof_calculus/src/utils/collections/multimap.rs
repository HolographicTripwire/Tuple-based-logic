use std::{collections::{HashMap, HashSet}, hash::Hash};

pub struct MultiMap<K:Hash,V:Hash>{
    inner: HashMap<K,HashSet<V>>
}

impl <K:Hash+Eq,V:Hash+Eq> MultiMap<K,V> {
    pub fn new() -> Self { Self{inner: HashMap::new()} }
    
    pub fn insert(&mut self, key: K, value: V) -> bool {
        match self.inner.get_mut(&key) {
            Some(set) => { set.insert(value) },
            None => {
                self.inner.insert(key, HashSet::from_iter([value]));
                true
            }
        }
    }

    pub fn remove(&mut self, key: &K, value: &V) -> bool {
        match self.inner.get_mut(key) {
            Some(set) => { if set.remove(value) {
                if set.len() == 0 { self.inner.remove(key); }
                true
            } else { false }},
            None => false,
        }
    }

    pub fn get(&self, key: &K) -> Option<&HashSet<V>> { self.inner.get(key) }
    pub fn get_refs(&self, key: &K) -> Option<HashSet<&V>> { self.inner.get(key).map(|set| set.iter().collect()) }
    pub fn keys(&self) -> std::collections::hash_map::Keys<'_, K, HashSet<V>> { self.inner.keys() }
    pub fn values(&self) -> impl IntoIterator<Item=&HashSet<V>> { self.inner.values() }
    pub fn flat_values(&self) -> impl IntoIterator<Item=&V> { self.inner.values().flat_map(|x| x) }
    pub fn into_values(self) -> impl IntoIterator<Item=HashSet<V>> { self.inner.into_values() }
}
impl <K:Hash,V:Hash> IntoIterator for MultiMap<K,V> {
    type Item = (K,HashSet<V>);
    type IntoIter = std::collections::hash_map::IntoIter<K, HashSet<V>>;

    fn into_iter(self) -> Self::IntoIter { self.inner.into_iter() }
}