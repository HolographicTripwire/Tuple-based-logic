use std::hash::Hash;
use std::collections::HashMap;

use crate::utils::collections::maps::KeyConflictError;

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
