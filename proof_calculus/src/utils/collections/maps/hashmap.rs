use std::hash::Hash;
use std::collections::HashMap;

use crate::utils::collections::maps::KeyConflictError;

const UNASSIGNED_JUST_INSERTED_VALUE_EXCEPTION: &str = "A value was just inserted into HashMap at a particular key, yet the key remains unassigned";
pub fn combine_hashmaps_without_conflicts<K:Clone+Eq+Hash,V: PartialEq<V>,I: IntoIterator<Item = HashMap<K,V>>>(assignments: I) -> Result<HashMap<K,V>,KeyConflictError<K,V>> {
    let mut assignments = assignments.into_iter();
    let Some(mut map1) = assignments.next() else { return Ok(HashMap::default()) };
    for map2 in assignments {
        let (smaller, mut larger) =  if map1.len() < map2.len() {(map1, map2)} else {(map2,map1)};
        for (k,v) in smaller { 
            if let Some(v2) = larger.insert(k.clone(), v) {
                let v1 = larger.remove(&k).expect(UNASSIGNED_JUST_INSERTED_VALUE_EXCEPTION);
                return Err(KeyConflictError::new(k,v1,v2))
            };
        }
        map1 = larger;
    }
    Ok(map1)
}
