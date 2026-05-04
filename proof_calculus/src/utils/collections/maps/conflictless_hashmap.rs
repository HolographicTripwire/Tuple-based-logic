use std::hash::Hash;
use std::collections::HashMap;

use crate::utils::collections::maps::KeyConflictError;
use crate::utils::traits::combinable::TryCombine;
use crate::utils::traits::try_from_iter::TryFromIterator;

const UNASSIGNED_JUST_INSERTED_VALUE_EXCEPTION: &str = "A value was just inserted into HashMap at a particular key, yet the key remains unassigned";
pub fn insert_into_hashmap_without_conflicts<K:Clone+Eq+Hash,V:PartialEq<V>,I: IntoIterator<Item=(K,V)>>(mut insert_into: HashMap<K,V>, insert_from: I) -> Result<HashMap<K,V>,KeyConflictError<K,V>> {
    for (k,v) in insert_from { 
        if let Some(v2) = insert_into.insert(k.clone(), v) {
            let v1 = insert_into.remove(&k).expect(UNASSIGNED_JUST_INSERTED_VALUE_EXCEPTION);
            return Err(KeyConflictError::new(k,v1,v2))
        };
    } Ok(insert_into)
}

pub fn create_hashmap_without_conflicts<K:Clone+Eq+Hash,V:PartialEq<V>,I: IntoIterator<Item=(K,V)>>(pairs: I) -> Result<HashMap<K,V>,KeyConflictError<K,V>> {
    let map = HashMap::new();
    insert_into_hashmap_without_conflicts(map, pairs)
}

pub fn combine_hashmaps_without_conflicts<K:Clone+Eq+Hash,V: PartialEq<V>,I: IntoIterator<Item = HashMap<K,V>>>(hashmaps: I) -> Result<HashMap<K,V>,KeyConflictError<K,V>> {
    let mut hashmaps = hashmaps.into_iter();
    let Some(mut map1) = hashmaps.next() else { return Ok(HashMap::default()) };
    for map2 in hashmaps {
        let (smaller, larger) =  if map1.len() < map2.len() {(map1, map2)} else {(map2,map1)};
        map1 = insert_into_hashmap_without_conflicts(larger, smaller)?;
    } Ok(map1)
}

#[derive(PartialEq,Eq,Clone,Debug)]
pub struct ConflictlessHashMap<K: Eq + Hash,V: PartialEq<V>>(HashMap<K,V>);
impl <K: Eq + Hash,V: PartialEq<V>> ConflictlessHashMap<K,V> {
    pub fn get(&self, key: &K) -> Option<&V> { self.0.get(key) }
    pub fn insert(&mut self, key: K, value: V) -> Result<(), KeyConflictError<K, V>> where K: Clone, V: Clone
        { self.insert_all([(key,value)]) }
    pub fn insert_all<I: IntoIterator<Item=(K,V)>>(&mut self, iter: I) -> Result<(),KeyConflictError<K,V>> where K: Clone, V: Clone { 
        match insert_into_hashmap_without_conflicts(self.0.clone(), iter) {
            Ok(after) => { self.0 = after; Ok(()) },
            Err(conflict) => Err(conflict),
        }
    }
    pub fn iter(&self) -> impl Iterator<Item = (&K,&V)> { self.0.iter() }
}

impl <K: Eq + Hash, V: PartialEq<V>> Default for ConflictlessHashMap<K,V>
    { fn default() -> Self { Self(Default::default()) } }
impl <K: Eq + Hash,V: PartialEq<V>> Into<HashMap<K,V>> for ConflictlessHashMap<K,V>
    { fn into(self) -> HashMap<K,V> { self.0 } }
impl <K: Eq + Hash,V: PartialEq<V>> From<HashMap<K,V>> for ConflictlessHashMap<K,V>
    { fn from(value: HashMap<K,V>) -> Self { Self(value) } }
impl <K: Clone + Eq + Hash,V: PartialEq<V>> TryCombine for ConflictlessHashMap<K,V> {
    type CombinationError = KeyConflictError<K,V>;
    fn combine<I: IntoIterator<Item = Self>>(assignments: I) -> Result<Self,Self::CombinationError> {
        let hashmaps = assignments.into_iter().map(|v| v.0);
        let combined_hashmap = combine_hashmaps_without_conflicts(hashmaps)?;
        Ok(Self(combined_hashmap))
    }
}
impl <K: Clone + Eq + Hash,V: PartialEq<V>> TryFromIterator<(K,V)> for ConflictlessHashMap<K,V> {
    type Error = KeyConflictError<K,V>;
    fn try_from_iter<T: IntoIterator<Item = (K,V)>>(iter: T) -> Result<Self,Self::Error> {
        Ok(Self(create_hashmap_without_conflicts(iter)?))
    }
}