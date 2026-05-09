use itertools::Itertools;
use std::hash::Hash;

use crate::utils::{
    collections::{
        iterators::split_into_max_by_key,
        maps::{
            conflictless::KeyConflictError,
            dense_usize_map::{
                DenseUsizeMap, KEYSET_VALUE_OUT_OF_BOUNDS_EXCEPTION,
                UNASSIGNED_JUST_INSERTED_VALUE_EXCEPTION,
            },
        },
    },
    traits::{combinable::TryCombine, try_from_iter::TryFromIterator},
};

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ConflictlessDenseUsizeMap<K: Clone + Eq + Hash + Into<usize>, V: PartialEq<V>>(
    DenseUsizeMap<K, V>,
);

impl<K: Clone + Eq + Hash + Into<usize>, V: PartialEq<V>> ConflictlessDenseUsizeMap<K, V> {
    pub fn get(&self, key: &K) -> Option<&V> {
        self.0.get(key)
    }
    pub fn get_mut(&mut self, key: &K) -> Option<&mut V> {
        self.0.get_mut(key)
    }

    pub fn iter(&self) -> impl Iterator<Item = (K, &V)> {
        self.0.iter()
    }
    pub fn from_iter_unchecked<I: IntoIterator<Item = (K, V)>>(pairs: I) -> Self {
        Self(DenseUsizeMap::from_iter(pairs))
    }

    pub fn insert(&mut self, key: K, value: V) -> Result<(), KeyConflictError<K, V>> {
        match self.0.insert(key.clone(), value) {
            Some(old_value) => {
                let new_value = self
                    .0
                    .inner_vec()
                    .get_mut(key.clone().into())
                    .expect(UNASSIGNED_JUST_INSERTED_VALUE_EXCEPTION);
                // A value was just inserted, so panic if it's no longer there
                debug_assert!(
                    new_value.is_some(),
                    "{}",
                    UNASSIGNED_JUST_INSERTED_VALUE_EXCEPTION
                );
                // Check if the value is the same as it was before, and return Err() containing the conflict otherwise
                if let Some(new_value) = new_value.take_if(|v| v == &old_value) {
                    Err(KeyConflictError::new(key.clone(), old_value, new_value))
                } else {
                    Ok(())
                }
            }
            None => Ok(()),
        }
    }

    pub fn transform_values<V2: PartialEq<V2>, F: Fn(&V) -> V2>(
        &self,
        transformer: F,
    ) -> ConflictlessDenseUsizeMap<K, V2> {
        ConflictlessDenseUsizeMap(self.0.transform_values(transformer))
    }
    pub fn try_transform_values<Err, V2: PartialEq<V2>, F: Fn(&V) -> Result<V2, Err>>(
        &self,
        transformer: F,
    ) -> Result<ConflictlessDenseUsizeMap<K, V2>, (K, Err)>
    where
        K: From<usize>,
    {
        Ok(ConflictlessDenseUsizeMap(
            self.0.try_transform_values(transformer)?,
        ))
    }
}

impl<K: Clone + Eq + Hash + Into<usize>, V: PartialEq<V>> Default
    for ConflictlessDenseUsizeMap<K, V>
{
    fn default() -> Self {
        Self(Default::default())
    }
}

impl<K: Clone + Eq + Hash + Into<usize>, V: PartialEq<V>> TryCombine
    for ConflictlessDenseUsizeMap<K, V>
{
    type CombinationError = KeyConflictError<K, V>;

    fn try_combine<I: IntoIterator<Item = Self>>(maps: I) -> Result<Self, Self::CombinationError> {
        // Use the largest map as a starting point
        let (mut largest, remaining) = match split_into_max_by_key(maps, |m| m.0.inner_vec().len())
        {
            Some(v) => v,
            None => return Ok(Self::default()),
        };
        // Fill the largest map with values from the smaller maps
        for map in remaining {
            for (key, value) in map {
                insert_within_bounds_into_vec_without_conflicts(
                    &mut largest.0.inner_vec(),
                    key,
                    value,
                )?;
            }
        }
        // Return the modified largest map
        Ok(largest)
    }
}
impl<K: Clone + Eq + Hash + Into<usize>, V: PartialEq<V>> TryFromIterator<(K, V)>
    for ConflictlessDenseUsizeMap<K, V>
{
    type Error = KeyConflictError<K, V>;
    fn try_from_iter<T: IntoIterator<Item = (K, V)>>(iter: T) -> Result<Self, Self::Error> {
        Self::try_from(iter.into_iter().collect_vec())
    }
}
impl<K: Clone + Eq + Hash + Into<usize>, V: PartialEq<V>> TryFrom<Vec<(K, V)>>
    for ConflictlessDenseUsizeMap<K, V>
{
    type Error = KeyConflictError<K, V>;
    fn try_from(pairs: Vec<(K, V)>) -> Result<Self, Self::Error> {
        match pairs
            .iter()
            .map(|(key, _value)| key.clone())
            .max_by_key(|k| <K as Into<usize>>::into(k.clone()))
        {
            Some(items) => {
                let mut values: Vec<Option<V>> = (0..items.into()).map(|_| None).collect();
                let assigned = pairs.iter().map(|(key, _value)| key.clone()).collect();
                for (key, value) in pairs {
                    insert_within_bounds_into_vec_without_conflicts(&mut values, key, value)?
                }
                Ok(Self(DenseUsizeMap::new_unchecked(assigned, values)))
            }
            None => Ok(Self::default()),
        }
    }
}
impl<K: Clone + Eq + Hash + Into<usize>, V: PartialEq<V>> IntoIterator
    for ConflictlessDenseUsizeMap<K, V>
{
    type Item = (K, V);
    type IntoIter = std::vec::IntoIter<(K, V)>;
    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

fn insert_within_bounds_into_vec_without_conflicts<
    K: Clone + Eq + Hash + Into<usize>,
    V: PartialEq<V>,
>(
    values: &mut Vec<Option<V>>,
    key: K,
    value: V,
) -> Result<(), KeyConflictError<K, V>> {
    // Insert the value
    match std::mem::replace(
        values
            .get_mut(key.clone().into())
            .expect(KEYSET_VALUE_OUT_OF_BOUNDS_EXCEPTION),
        Some(value),
    ) {
        // If there was already a value there
        Some(old_value) => {
            // A value was just inserted, so panic if it's no longer there
            let new_value = match values.get_mut(key.clone().into()) {
                Some(v) => v,
                _ => panic!("{}", UNASSIGNED_JUST_INSERTED_VALUE_EXCEPTION),
            };
            debug_assert!(
                new_value.is_some(),
                "{}",
                UNASSIGNED_JUST_INSERTED_VALUE_EXCEPTION
            );
            // Check if the value is the same as it was before, and return Err() containing the conflict otherwise
            if let Some(new_value) = new_value.take_if(|v| v == &old_value) {
                Err(KeyConflictError::new(key, old_value, new_value))
            } else {
                Ok(())
            }
        }
        None => Ok(()),
    }
}
