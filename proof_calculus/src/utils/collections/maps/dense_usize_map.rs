use std::{hash::Hash, collections::HashSet};

use itertools::Itertools;

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

pub (super) const KEYSET_VALUE_OUT_OF_BOUNDS_EXCEPTION: &str = "DenseUsizeMap's 'assigned' keyset included an out-of-bounds key";
pub (super) const KEYSET_VALUE_UNASSIGNED_EXCEPTION: &str = "DenseUsizeMap's 'assigned' keyset included an unassigned key";
pub (super) const UNASSIGNED_JUST_INSERTED_VALUE_EXCEPTION: &str = "A value was just inserted into DenseUsizeMap at a particular key, yet the key remains unassigned";

impl<K: Clone + Eq + Hash + Into<usize>, V> DenseUsizeMap<K,V> {
    pub (super) fn new_unchecked(assigned: HashSet<K>, values: Vec<Option<V>>) -> Self
        { Self { assigned, values } }
    pub (super) fn inner_vec(&mut self) -> &mut Vec<Option<V>> { &mut self.values }
    pub (super) fn inner_set(&mut self) -> &mut HashSet<K> { &mut self.assigned }
    
    pub fn keys(&self) -> &HashSet<K> { &self.assigned }

    pub fn get(&self, key: &K) -> Option<&V> { match self.values.get(key.clone().into()) {
        Some(Some(v)) => Some(v),
        _ => None,
    }}
    pub fn get_mut(&mut self, key: &K) -> Option<&mut V> {match self.values.get_mut(key.clone().into()) {
        Some(Some(v)) => Some(v),
        _ => None,
    }}
    pub fn insert(&mut self, key: K, value: V) -> Option<V> { match self.values.get_mut(key.clone().into()) {
        Some(v) => std::mem::replace(v, Some(value)),
        None => {
            self.values.resize_with(key.clone().into(), || None);
            self.values[key.clone().into()] = Some(value);
            self.assigned.insert(key);
            None
        },
    }}
    pub fn remove(&mut self, key: K) -> Option<V> { match self.values.get_mut(key.clone().into()) {
        Some(v) => {
            self.assigned.remove(&key);
            std::mem::take(v)
        },
        None => None,
    }}

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
impl <K: Clone + Eq + Hash + Into<usize>, V> FromIterator<(K,V)> for DenseUsizeMap<K,V> {
    fn from_iter<T: IntoIterator<Item = (K,V)>>(pairs: T) -> Self {
        // Get the largest key
        let v: Vec<_> = pairs.into_iter().collect();
        match v.iter().map(|(key,_value)| key.clone()).max_by_key(|k| <K as Into<usize>>::into(k.clone())) {
            // If a largest key is available...
            Some(items) => {
                // ...Create structures capable of supporting keys up to the largest one.
                let mut values: Vec<Option<V>> = (0..items.into()).map(|_| None).collect();
                let assigned = v.iter().map(|(key,_value)| key.clone()).collect();
                // Insert all values
                for (key,value) in v { 
                    values[key.into()] = Some(value);
                } Self{assigned, values}
            },
            // If there is no largest key, then there must have been no pairs at all, so return an empty structure
            None => Self::default(),
        }
    }
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
