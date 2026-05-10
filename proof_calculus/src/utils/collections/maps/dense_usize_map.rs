use std::{collections::HashSet, hash::Hash};

use itertools::Itertools;

use crate::utils::traits::map::{Map, MapWithTransformableValues, MapWithoutConflicts};

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct DenseUsizeMap<K: Clone + Eq + Hash + Into<usize>, V> {
    assigned: HashSet<K>, // Keeps track of what entries are actually assigned a value. Used for iteration
    values: Vec<Option<V>>, // The values themselves. None indicates no value for the key at that index.
}
impl<K: Clone + Eq + Hash + Into<usize>, V> Default for DenseUsizeMap<K, V> {
    fn default() -> Self {
        Self {
            assigned: Default::default(),
            values: Default::default(),
        }
    }
}

pub(super) const KEYSET_VALUE_OUT_OF_BOUNDS_EXCEPTION: &str =
    "DenseUsizeMap's 'assigned' keyset included an out-of-bounds key";
pub(super) const KEYSET_VALUE_UNASSIGNED_EXCEPTION: &str =
    "DenseUsizeMap's 'assigned' keyset included an unassigned key";

impl<K: Clone + Eq + Hash + Into<usize>, V> DenseUsizeMap<K, V> {
    pub fn keys(&self) -> &HashSet<K> {
        &self.assigned
    }
}
impl<K: Clone + Eq + Hash + Into<usize>, V> Map<K, V> for DenseUsizeMap<K, V> {
    fn get(&self, key: &K) -> Option<&V> {
        match self.values.get(key.clone().into()) {
            Some(Some(v)) => Some(v),
            _ => None,
        }
    }
    fn get_mut(&mut self, key: &K) -> Option<&mut V> {
        match self.values.get_mut(key.clone().into()) {
            Some(Some(v)) => Some(v),
            _ => None,
        }
    }
    fn insert(&mut self, key: K, value: V) -> Option<V> {
        match self.values.get_mut(key.clone().into()) {
            Some(v) => std::mem::replace(v, Some(value)),
            None => {
                self.values.resize_with(key.clone().into(), || None);
                self.values[key.clone().into()] = Some(value);
                self.assigned.insert(key);
                None
            }
        }
    }

    fn iter<'a>(&'a self) -> impl Iterator<Item = (&'a K, &'a V)>
    where
        K: 'a,
        V: 'a,
    {
        self.assigned
            .iter()
            .map(|key| match self.values.get(key.clone().into()) {
                Some(Some(v)) => (key, v),
                Some(None) => panic!("{}", KEYSET_VALUE_UNASSIGNED_EXCEPTION),
                None => panic!("{}", KEYSET_VALUE_OUT_OF_BOUNDS_EXCEPTION),
            })
    }

    fn remove(&mut self, key: &K) -> Option<V> {
        match self.values.get_mut(key.clone().into()) {
            Some(v) => {
                self.assigned.remove(&key);
                std::mem::take(v)
            }
            None => None,
        }
    }
}
impl<K: Clone + Eq + Hash + Into<usize>,V: PartialEq<V>> MapWithoutConflicts<K,V> for DenseUsizeMap<K,V> {
    fn insert_conflictless(&mut self, key: K, value: V) -> Result<(), super::KeyConflictError<K, V>>
    where
        V: PartialEq<V> {
        
    }

    fn try_combine_conflictless<I: IntoIterator<Item = Self>>(
        maps: I,
    ) -> Result<Self, super::KeyConflictError<K, V>> {
        todo!()
    }

    fn try_from_iter_without_conflicts<T: IntoIterator<Item = (K, V)>>(
        iter: T,
    ) -> Result<Self, super::KeyConflictError<K, V>> {
        todo!()
    }
} 
impl<K: Clone + Eq + Hash + Into<usize> + From<usize>, V1, V2> MapWithTransformableValues<K, V1, V2>
    for DenseUsizeMap<K, V1>
{
    type SelfTransformed = DenseUsizeMap<K, V2>;
    fn with_values_transformed<F: Fn(&V1) -> V2>(&self, transformer: F) -> DenseUsizeMap<K, V2> {
        DenseUsizeMap {
            assigned: self.assigned.clone(),
            values: self
                .values
                .iter()
                .map(|v| match v {
                    Some(value) => Some((transformer)(value)),
                    None => None,
                })
                .collect(),
        }
    }
    fn try_with_values_transformed<Err, F: Fn(&V1) -> Result<V2, Err>>(
        &self,
        transformer: F,
    ) -> Result<DenseUsizeMap<K, V2>, (K, Err)> {
        Ok(DenseUsizeMap {
            assigned: self.assigned.clone(),
            values: self
                .values
                .iter()
                .enumerate()
                .map(|(k, v)| {
                    Ok(match v {
                        Some(value) => Some((transformer)(value).map_err(|err| (k.into(), err))?),
                        None => None,
                    })
                })
                .try_collect()?,
        })
    }
}

impl<K: Clone + Eq + Hash + Into<usize>, V> FromIterator<(K, V)> for DenseUsizeMap<K, V> {
    fn from_iter<T: IntoIterator<Item = (K, V)>>(pairs: T) -> Self {
        // Get the largest key
        let v: Vec<_> = pairs.into_iter().collect();
        match v
            .iter()
            .map(|(key, _value)| key.clone())
            .max_by_key(|k| <K as Into<usize>>::into(k.clone()))
        {
            // If a largest key is available...
            Some(items) => {
                // ...Create structures capable of supporting keys up to the largest one.
                let mut values: Vec<Option<V>> = (0..items.into()).map(|_| None).collect();
                let assigned = v.iter().map(|(key, _value)| key.clone()).collect();
                // Insert all values
                for (key, value) in v {
                    values[key.into()] = Some(value);
                }
                Self { assigned, values }
            }
            // If there is no largest key, then there must have been no pairs at all, so return an empty structure
            None => Self::default(),
        }
    }
}

impl<K: Clone + Eq + Hash + Into<usize>, V> IntoIterator for DenseUsizeMap<K, V> {
    type Item = (K, V);
    type IntoIter = <Vec<(K, V)> as IntoIterator>::IntoIter;

    fn into_iter(mut self) -> Self::IntoIter {
        self.assigned
            .into_iter()
            .map(|key| {
                let value = self
                    .values
                    .get_mut(key.clone().into())
                    .expect(KEYSET_VALUE_OUT_OF_BOUNDS_EXCEPTION);
                (
                    key,
                    std::mem::take(value).expect(KEYSET_VALUE_UNASSIGNED_EXCEPTION),
                )
            })
            .collect_vec()
            .into_iter()
    }
}
