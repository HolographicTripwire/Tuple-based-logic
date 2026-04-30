use std::hash::Hash;
use std::collections::HashMap;

pub mod hashmap;
pub mod dense_usize_map;
pub mod multimap;

pub struct KeyConflictError<K,V: PartialEq<V>>{
    pub key: K,
    pub value1: V,
    pub value2: V
}
impl <K,V: PartialEq<V>> KeyConflictError<K,V> {
    pub fn new(key: K, value1: V, value2: V) -> Self {
        debug_assert!(value1 != value2);
        Self { key, value1, value2 }
    }
}

