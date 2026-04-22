mod atom;
mod compound;
mod duplication;

use std::{collections::{HashMap, HashSet}, hash::Hash};

pub use atom::TblExpressionTrackerBoundsAtomExactValue;
pub use compound::TblExpressionTrackerCompoundLengthBounds;
pub use duplication::TblExpressionTrackerDuplicationBounds;
use proof_calculus::utils::collections::multimap::MultiMap;

fn get_helper<'a,K1: Hash + Eq + Clone,K2: Hash + Eq,V: Hash + Eq>(map: &'a HashMap<K1,MultiMap<K2,V>>, key1: &K1, key2: &K2) -> HashSet<&'a V> {
    let optional_found = map.get(key1)
        .map(|inner| inner.get_refs(&key2));
    if let Some(Some(found)) = optional_found { found } else { HashSet::new() }
}

fn insert_helper<K1: Hash + Eq + Clone,K2: Hash + Eq,V: Hash + Eq>(map: &mut HashMap<K1,MultiMap<K2,V>>, key1: &K1, key2: K2, value: V) -> bool {
    match map.get_mut(&key1) {
        Some(inner) => inner.insert(key2, value),
        None => {
            let mut inner = MultiMap::new();
            inner.insert(key2, value);
            map.insert(key1.clone(), inner);
            true
        },
    }
}
