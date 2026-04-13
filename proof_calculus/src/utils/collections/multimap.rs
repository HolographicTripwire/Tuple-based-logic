use std::{collections::{HashMap, HashSet}, hash::Hash};

pub struct MultiMap<K:Hash,V:Hash>(HashMap<K,HashSet<V>>);

impl <K:Hash+Eq,V:Hash+Eq> MultiMap<K,V> {
    pub fn new() -> Self { Self(HashMap::new()) }
    
    pub fn insert(&mut self, key: K, value: V) {
        match self.0.get_mut(&key) {
            Some(set) => { set.insert(value); },
            None => { self.0.insert(key, HashSet::from_iter([value])); },
        };
    }

    pub fn remove(&mut self, key: &K, value: &V) -> bool {
        match self.0.get_mut(key) {
            Some(set) => { if set.remove(value) {
                if set.len() == 0 { self.0.remove(key); }
                true
            } else { false }},
            None => false,
        }
    }

    pub fn get(&mut self, key: &K) -> Option<&HashSet<V>> { self.0.get(key) }
}
impl <K:Hash,V:Hash> IntoIterator for MultiMap<K,V> {
    type Item = (K,HashSet<V>);
    type IntoIter = std::collections::hash_map::IntoIter<K, HashSet<V>>;

    fn into_iter(self) -> Self::IntoIter { self.0.into_iter() }
}