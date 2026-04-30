use std::collections::HashSet;

use itertools::Itertools;

use crate::utils::{collections::{iterators::split_into_max_by_key, maps::KeyConflictError}, traits::try_from_iter::TryFromIterator};

#[derive(Clone,PartialEq,Eq,Debug)]
pub struct DenseUsizeMap<V> {
    assigned: HashSet<usize>, // Keeps track of what entries are actually assigned a value. Used for iteration
    values: Vec<Option<V>> // The values themselves. None indicates no value for the key at that index.
}
impl <V> Default for DenseUsizeMap<V> {
    fn default() -> Self { Self{
        assigned: Default::default(),
        values: Default::default()
    } }
}

const KEYSET_VALUE_OUT_OF_BOUNDS_EXCEPTION: &str = "DenseUsizeMap's 'assigned' keyset included an out-of-bounds key";
const KEYSET_VALUE_UNASSIGNED_EXCEPTION: &str = "DenseUsizeMap's 'assigned' keyset included an unassigned key";
const UNASSIGNED_JUST_INSERTED_VALUE_EXCEPTION: &str = "A value was just inserted into DenseUsizeMap at a particular key, yet the key remains unassigned";

impl<V> DenseUsizeMap<V> {
    pub fn insert(&mut self, key: usize, value: V) -> Option<V> { match self.values.get_mut(key) {
        Some(v) => std::mem::replace(v, Some(value)),
        None => {
            self.values.resize_with(key, || None);
            self.values[key] = Some(value);
            self.assigned.insert(key);
            None
        },
    }}
    
    pub fn get(&self, key: usize) -> Option<&V> { match self.values.get(key) {
        Some(Some(v)) => Some(v),
        _ => None,
    }}
    pub fn remove(&mut self, key: usize) -> Option<V> { match self.values.get_mut(key) {
        Some(v) => {
            self.assigned.remove(&key);
            std::mem::take(v)
        },
        None => None,
    }}

    /// Merges some [DenseUsizeMap] objects, combining their key-value pairs.
    /// If the same key is assigned two different values, throws an [Err] containing the key and the two overlapping values that were assigned
    /// Note that this means that only the first conflict is returned
    pub fn merge_without_conflicts<I: IntoIterator<Item=Self>>(maps: I) -> Result<Self,KeyConflictError<usize,V>> where V: PartialEq<V> {
        // Use the largest map as a starting point
        let (mut largest, remaining) = match split_into_max_by_key(maps, |m| m.values.len()) {
            Some(v) => v,
            None => return Ok(DenseUsizeMap::default()),
        };
        // Fill the largest map with values from the smaller maps
        for map in remaining {
            for (key, value) in map {
                insert_within_bounds_into_vec_without_conflicts(&mut largest.values, key, value)?;
            }
        }
        // Return the modified largest map
        Ok(largest)
    }

    pub fn iter(&self) -> impl IntoIterator<Item=(usize,&V)> {
        self.assigned
            .iter()
            .map(|key| 
                match self.values.get(*key) {
                    Some(Some(v)) => (*key,v),
                    Some(None) => panic!("{}",KEYSET_VALUE_UNASSIGNED_EXCEPTION),
                    None => panic!("{}",KEYSET_VALUE_OUT_OF_BOUNDS_EXCEPTION),
                }
            )
    }
}
impl <V> IntoIterator for DenseUsizeMap<V> {
    type Item = (usize,V);
    type IntoIter = <Vec<(usize,V)> as IntoIterator>::IntoIter;

    fn into_iter(mut self) -> Self::IntoIter {
        self.assigned
            .iter()
            .map(|key| { 
                let value = self.values
                    .get_mut(*key)
                    .expect(KEYSET_VALUE_OUT_OF_BOUNDS_EXCEPTION);
                (*key, std::mem::take(value).expect(KEYSET_VALUE_UNASSIGNED_EXCEPTION))
            }).collect_vec().into_iter()
    }
}

impl <V: PartialEq<V>> TryFromIterator<(usize,V)> for DenseUsizeMap<V> {
    type Error = KeyConflictError<usize,V>;
    fn try_from_iter<T: IntoIterator<Item = (usize,V)>>(iter: T) -> Result<Self,Self::Error> {
        Self::try_from(iter.into_iter().collect_vec())
    }
}
impl <V: PartialEq<V>> TryFrom<Vec<(usize,V)>> for DenseUsizeMap<V> {
    type Error = KeyConflictError<usize,V>;
    fn try_from(pairs: Vec<(usize,V)>) -> Result<Self,Self::Error> {
        match pairs.iter().map(|(key,_value)| key).max() {
            Some(items) => {
                let mut values: Vec<Option<V>> = (0..*items).map(|_| None).collect();
                let assigned = pairs.iter().map(|(key,_value)| *key).collect();
                for (key,value) in pairs { 
                    insert_within_bounds_into_vec_without_conflicts(&mut values, key, value)?
                } Ok(Self{assigned, values})
            },
            None => Ok(Self::default()),
        }
    }
}


fn insert_within_bounds_into_vec_without_conflicts<V: PartialEq<V>>(values: &mut Vec<Option<V>>, key: usize, value: V) -> Result<(),KeyConflictError<usize,V>> {
    // Insert the value
    match std::mem::replace(values.get_mut(key).expect(KEYSET_VALUE_OUT_OF_BOUNDS_EXCEPTION), Some(value)) {
        // If there was already a value there
        Some(old_value) => { 
            // A value was just inserted, so panic if it's no longer there
            let new_value = match values.get_mut(key) {
                Some(v) => v,
                _ => panic!("{}",UNASSIGNED_JUST_INSERTED_VALUE_EXCEPTION)
            };
            debug_assert!(new_value.is_some(), "{}", UNASSIGNED_JUST_INSERTED_VALUE_EXCEPTION);
            // Check if the value is the same as it was before, and return Err() containing the conflict otherwise
            if let Some(new_value) = new_value.take_if(|v| v == &old_value)
                { Err(KeyConflictError::new(key,old_value,new_value)) }
            else { Ok(()) }
        }, None => Ok(())
    }
}
