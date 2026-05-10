use std::collections::HashMap;
use std::hash::Hash;

use itertools::Itertools;

use crate::utils::{
    collections::maps::KeyConflictError,
    traits::map::{
        Map, MapWithTransformableValues, MapWithoutConflicts,
        UNASSIGNED_JUST_INSERTED_VALUE_EXCEPTION,
    },
};

impl<K: Hash + Eq, V> Map<K, V> for HashMap<K, V> {
    fn get(&self, key: &K) -> Option<&V> {
        HashMap::get(self, key)
    }

    fn get_mut(&mut self, key: &K) -> Option<&mut V> {
        HashMap::get_mut(self, key)
    }

    fn insert(&mut self, key: K, value: V) -> Option<V> {
        HashMap::insert(self, key, value)
    }
    fn remove(&mut self, k: &K) -> Option<V> {
        self.remove(&k)
    }

    fn iter<'a>(&'a self) -> impl Iterator<Item = (&'a K, &'a V)>
    where
        K: 'a,
        V: 'a,
    {
        HashMap::iter(self)
    }
}
impl<K: Hash + Clone + Eq, V1, V2> MapWithTransformableValues<K, V1, V2> for HashMap<K, V1> {
    type SelfTransformed = HashMap<K, V2>;

    fn with_values_transformed<F: Fn(&V1) -> V2>(&self, transformer: F) -> Self::SelfTransformed {
        HashMap::iter(self)
            .map(|(k, v)| (k.clone(), (transformer)(v)))
            .collect()
    }

    fn try_with_values_transformed<Err, F: Fn(&V1) -> Result<V2, Err>>(
        &self,
        transformer: F,
    ) -> Result<Self::SelfTransformed, (K, Err)> {
        HashMap::iter(self)
            .map(|(k, v1)| match (transformer)(v1) {
                Ok(v2) => Ok((k.clone(), v2)),
                Err(err) => Err((k.clone(), err)),
            })
            .try_collect()
    }
}
impl<K: Hash + Eq + Clone, V: PartialEq<V>> MapWithoutConflicts<K, V> for HashMap<K, V> {
    fn insert_conflictless(&mut self, key: K, value: V) -> Result<(), KeyConflictError<K, V>>
    where
        V: PartialEq<V>,
    {
        if let Some(v2) = self.insert(key.clone(), value) {
            let v1 = self
                .remove(&key)
                .expect(UNASSIGNED_JUST_INSERTED_VALUE_EXCEPTION);
            return Err(KeyConflictError::new(key, v1, v2));
        };
        Ok(())
    }

    fn try_combine_conflictless<I: IntoIterator<Item = Self>>(
        hashmaps: I,
    ) -> Result<Self, KeyConflictError<K, V>> {
        let mut hashmaps = hashmaps.into_iter();
        let Some(mut map1) = hashmaps.next() else {
            return Ok(HashMap::default());
        };
        for map2 in hashmaps {
            let (smaller, mut larger) = if map1.len() < map2.len() {
                (map1, map2)
            } else {
                (map2, map1)
            };
            larger.insert_all_conflictless(smaller)?;
            map1 = larger;
        }
        Ok(map1)
    }

    fn try_from_iter_without_conflicts<T: IntoIterator<Item = (K, V)>>(
        iter: T,
    ) -> Result<Self, KeyConflictError<K, V>> {
        let mut map = HashMap::new();
        map.insert_all_conflictless(iter)?;
        Ok(map)
    }
}
