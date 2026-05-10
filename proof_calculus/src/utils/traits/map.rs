use crate::utils::collections::maps::KeyConflictError;

pub const UNASSIGNED_JUST_INSERTED_VALUE_EXCEPTION: &str =
    "A value was just inserted into a Map at a particular key, yet the key remains unassigned";

pub trait Map<K, V> {
    fn get(&self, key: &K) -> Option<&V>;
    fn get_mut(&mut self, key: &K) -> Option<&mut V>;

    fn insert(&mut self, key: K, value: V) -> Option<V>;
    fn remove(&mut self, k: &K) -> Option<V>;

    fn iter<'a>(&'a self) -> impl Iterator<Item = (&'a K, &'a V)>
    where
        K: 'a,
        V: 'a;
}

pub trait MapWithTransformableValues<K, V1, V2>: Map<K, V1> {
    type SelfTransformed: Map<K, V2>;

    fn with_values_transformed<F: Fn(&V1) -> V2>(&self, transformer: F) -> Self::SelfTransformed;
    fn try_with_values_transformed<Err, F: Fn(&V1) -> Result<V2, Err>>(
        &self,
        transformer: F,
    ) -> Result<Self::SelfTransformed, (K, Err)>;
}

pub trait MapWithoutConflicts<K, V: PartialEq<V>>: Sized + Map<K, V> {
    fn insert_conflictless(&mut self, key: K, value: V) -> Result<(), KeyConflictError<K, V>>
    where
        V: PartialEq<V>;
    fn insert_all_conflictless<I: IntoIterator<Item = (K, V)>>(
        &mut self,
        into_iter: I,
    ) -> Result<(), KeyConflictError<K, V>>
    where
        V: PartialEq<V>,
    {
        into_iter
            .into_iter()
            .map(|(k, v)| self.insert_conflictless(k, v))
            .collect()
    }

    fn try_combine_conflictless<I: IntoIterator<Item = Self>>(
        maps: I,
    ) -> Result<Self, KeyConflictError<K, V>>;

    fn try_from_iter_conflictless<T: IntoIterator<Item = (K, V)>>(
        iter: T,
    ) -> Result<Self, KeyConflictError<K, V>>;
}
