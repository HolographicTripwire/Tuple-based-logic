use std::{hash::Hash, collections::HashSet};

use itertools::Itertools;

use crate::utils::{collections::{iterators::split_into_max_by_key, maps::KeyConflictError}, traits::try_from_iter::TryFromIterator};

#[derive(Clone,PartialEq,Eq,Debug)]
pub struct DenseUsizeMap<K: Clone + Eq + Hash + Into<usize>,V> {
    assigned: HashSet<K>, // Keeps track of what entries are actually assigned a value. Used for iteration
    values: Vec<Option<V>> // The values themselves. None indicates no value for the key at that index.
}
impl <K: Clone + Eq + Hash + Into<usize>, V> Default for DenseUsizeMap<K,V> {
    fn default() -> Self { Self{
        assigned: Default::default(),
        values: Default::default()
    } }
}

const KEYSET_VALUE_OUT_OF_BOUNDS_EXCEPTION: &str = "DenseUsizeMap's 'assigned' keyset included an out-of-bounds key";
const KEYSET_VALUE_UNASSIGNED_EXCEPTION: &str = "DenseUsizeMap's 'assigned' keyset included an unassigned key";
const UNASSIGNED_JUST_INSERTED_VALUE_EXCEPTION: &str = "A value was just inserted into DenseUsizeMap at a particular key, yet the key remains unassigned";

impl<K: Clone + Eq + Hash + Into<usize>, V> DenseUsizeMap<K,V> {
    pub fn insert(&mut self, key: K, value: V) -> Option<V> { match self.values.get_mut(key.clone().into()) {
        Some(v) => std::mem::replace(v, Some(value)),
        None => {
            self.values.resize_with(key.clone().into(), || None);
            self.values[key.clone().into()] = Some(value);
            self.assigned.insert(key);
            None
        },
    }}
    
    pub fn get(&self, key: &K) -> Option<&V> { match self.values.get(key.clone().into()) {
        Some(Some(v)) => Some(v),
        _ => None,
    }}
    pub fn remove(&mut self, key: K) -> Option<V> { match self.values.get_mut(key.clone().into()) {
        Some(v) => {
            self.assigned.remove(&key);
            std::mem::take(v)
        },
        None => None,
    }}

    /// Merges some [DenseUsizeMap] objects, combining their key-value pairs.
    /// If the same key is assigned two different values, throws an [Err] containing the key and the two overlapping values that were assigned
    /// Note that this means that only the first conflict is returned
    pub fn merge_without_conflicts<I: IntoIterator<Item=Self>>(maps: I) -> Result<Self,KeyConflictError<K,V>> where V: PartialEq<V> {
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

    pub fn from_iter_unchecked<I: IntoIterator<Item=(K,V)>>(pairs: I) -> Self {
        let v: Vec<_> = pairs.into_iter().collect();
        match v.iter().map(|(key,_value)| key.clone()).max_by_key(|k| <K as Into<usize>>::into(k.clone())) {
            Some(items) => {
                let mut values: Vec<Option<V>> = (0..items.into()).map(|_| None).collect();
                let assigned = v.iter().map(|(key,_value)| key.clone()).collect();
                for (key,value) in v { 
                    values[key.into()] = Some(value);
                } Self{assigned, values}
            },
            None => Self::default(),
        }
    }
    pub fn iter(&self) -> impl Iterator<Item=(K,&V)> {
        self.assigned
            .iter()
            .map(|key| 
                match self.values.get(key.clone().into()) {
                    Some(Some(v)) => (key.clone(),v),
                    Some(None) => panic!("{}",KEYSET_VALUE_UNASSIGNED_EXCEPTION),
                    None => panic!("{}",KEYSET_VALUE_OUT_OF_BOUNDS_EXCEPTION),
                }
            )
    }

    pub fn transform_values<V2, F: Fn(&V) -> V2>(&self, transformer: F) -> DenseUsizeMap<K,V2> { DenseUsizeMap {
        assigned: self.assigned.clone(),
        values: self.values.iter().map(|v| match v {
            Some(value) => Some((transformer)(value)),
            None => None,
        }).collect()
    }}
    pub fn try_transform_values<Err,V2, F: Fn(&V) -> Result<V2,Err>>(&self, transformer: F) -> Result<DenseUsizeMap<K,V2>,(K,Err)> where K: From<usize> { Ok(DenseUsizeMap {
        assigned: self.assigned.clone(),
        values: self.values.iter().enumerate().map(|(k,v)| Ok(match v {
            Some(value) => Some((transformer)(value).map_err(|err| (k.into(),err))?),
            None => None,
        })).try_collect()?
    })}
}
impl <K: Clone + Eq + Hash + Into<usize>, V> IntoIterator for DenseUsizeMap<K,V> {
    type Item = (K,V);
    type IntoIter = <Vec<(K,V)> as IntoIterator>::IntoIter;

    fn into_iter(mut self) -> Self::IntoIter {
        self.assigned
            .into_iter()
            .map(|key| { 
                let value = self.values
                    .get_mut(key.clone().into())
                    .expect(KEYSET_VALUE_OUT_OF_BOUNDS_EXCEPTION);
                (key, std::mem::take(value).expect(KEYSET_VALUE_UNASSIGNED_EXCEPTION))
            }).collect_vec().into_iter()
    }
}

impl <K: Clone + Eq + Hash + Into<usize>, V: PartialEq<V>> TryFromIterator<(K,V)> for DenseUsizeMap<K,V> {
    type Error = KeyConflictError<K,V>;
    fn try_from_iter<T: IntoIterator<Item = (K,V)>>(iter: T) -> Result<Self,Self::Error> {
        Self::try_from(iter.into_iter().collect_vec())
    }
}
impl <K: Clone + Eq + Hash + Into<usize>, V: PartialEq<V>> TryFrom<Vec<(K,V)>> for DenseUsizeMap<K,V> {
    type Error = KeyConflictError<K,V>;
    fn try_from(pairs: Vec<(K,V)>) -> Result<Self,Self::Error> {
        match pairs.iter().map(|(key,_value)| key.clone()).max_by_key(|k| <K as Into<usize>>::into(k.clone())) {
            Some(items) => {
                let mut values: Vec<Option<V>> = (0..items.into()).map(|_| None).collect();
                let assigned = pairs.iter().map(|(key,_value)| key.clone()).collect();
                for (key,value) in pairs { 
                    insert_within_bounds_into_vec_without_conflicts(&mut values, key, value)?
                } Ok(Self{assigned, values})
            },
            None => Ok(Self::default()),
        }
    }
}


fn insert_within_bounds_into_vec_without_conflicts<K: Clone + Eq + Hash + Into<usize>, V: PartialEq<V>>(values: &mut Vec<Option<V>>, key: K, value: V) -> Result<(),KeyConflictError<K,V>> {
    // Insert the value
    match std::mem::replace(values.get_mut(key.clone().into()).expect(KEYSET_VALUE_OUT_OF_BOUNDS_EXCEPTION), Some(value)) {
        // If there was already a value there
        Some(old_value) => { 
            // A value was just inserted, so panic if it's no longer there
            let new_value = match values.get_mut(key.clone().into()) {
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
