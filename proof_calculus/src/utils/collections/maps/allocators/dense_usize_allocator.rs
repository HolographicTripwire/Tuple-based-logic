use std::collections::{BTreeSet, HashMap};
use std::hash::Hash;
use std::mem;

const LAST_VALUE_NONE_ERROR: &str = "DenseUsizeAllocator: last element in self.values was None";
const NO_LAST_VALUE_ERROR: &str = "DenseUsizeAllocator: there existed k such that self.len() - 1 == k, yet self.values had no last element";
const EXISTING_BUT_NOT_FOUND_ERROR: &str = "DenseUsizeAllocator: self.existing contained an element which could not be found in self.values";

#[derive(Clone, Eq, Debug)]
pub struct DenseUsizeAllocator<K: Clone + From<usize> + Into<usize>, V: Hash + Eq + Clone> {
    key_to_value: Vec<Option<V>>,
    value_to_key: HashMap<V, K>,
    missing_keys: BTreeSet<usize>,
}

impl<K: Clone + From<usize> + Into<usize>, V: Hash + Eq + Clone>
    PartialEq<DenseUsizeAllocator<K, V>> for DenseUsizeAllocator<K, V>
{
    fn eq(&self, other: &DenseUsizeAllocator<K, V>) -> bool {
        self.key_to_value == other.key_to_value // value_to_key and missing_keys can be fully determined by key_to_value
    }
}
impl<K: Clone + From<usize> + Into<usize>, V: Hash + Eq + Clone> Hash
    for DenseUsizeAllocator<K, V>
{
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.key_to_value.hash(state);
    }
}

impl<K: Clone + From<usize> + Into<usize>, V: Hash + Eq + Clone> DenseUsizeAllocator<K, V> {
    pub fn get_value_by_key(&self, key: &K) -> Option<&V> {
        match self.key_to_value.get(key.clone().into()) {
            Some(Some(v)) => Some(v),
            _ => None,
        }
    }
    pub fn get_key_by_value(&self, value: &V) -> Option<&K> {
        self.value_to_key.get(value)
    }

    pub fn insert_value(&mut self, value: V) -> (K, bool) {
        if let Some(key) = self.value_to_key.get(&value) {
            (key.clone(), false)
        } else if let Some(key) = self.missing_keys.first() {
            self.key_to_value[*key] = Some(value);
            ((*key).into(), true)
        } else {
            self.key_to_value.push(Some(value));
            (self.key_to_value.len().into(), true)
        }
    }
    pub fn remove_by_key(&mut self, key: K) -> Option<V> {
        let mut k = key.into();
        // If we are removing the last element:
        if k == self.key_to_value.len() - 1 {
            // Retrieve the element
            let value = self
                .key_to_value
                .pop()
                .expect(NO_LAST_VALUE_ERROR)
                .expect(LAST_VALUE_NONE_ERROR);
            self.value_to_key.remove(&value);
            // Shrink the map
            // 10 elements. remove element 9
            k -= 1; // 8
            while self.missing_keys.remove(&k) {
                k -= 1;
            }
            let tail = self.key_to_value.split_off(k);
            debug_assert!(tail.into_iter().all(|v| v.is_none()));
            // Return the element
            Some(value)
        }
        // If we are removing some element that is within bounds:
        else if k < self.key_to_value.len() {
            // Remove and return it,
            let value = mem::take(&mut self.key_to_value[k]);
            match value {
                Some(v) => {
                    self.missing_keys.insert(k);
                    self.value_to_key.remove(&v);
                    Some(v)
                }
                // Unless it's already missing
                None => None,
            }
        }
        // If we tried to remove an element that was out of bounds, just return None
        else {
            None
        }
    }
    pub fn remove_by_value(&mut self, value: &V) -> Option<K> {
        if let Some(key) = self.value_to_key.remove(value) {
            // Remove the value
            self.key_to_value[key.clone().into()] = None;
            // Return the index
            Some(key)
        } else {
            None
        }
    }

    pub fn contains_key(&self, key: &K) -> bool {
        match self.key_to_value.get(key.clone().into()) {
            Some(Some(_value)) => true,
            _ => false,
        }
    }
    pub fn contains_value(&self, value: &V) -> bool {
        self.value_to_key.contains_key(value)
    }
    pub fn allocated_len(&self) -> usize {
        self.key_to_value.len()
    }
    pub fn missing_keys_count(&self) -> usize {
        self.missing_keys.len()
    }
    pub fn allocated_keys_count(&self) -> usize {
        self.allocated_len() - self.missing_keys_count()
    }
}
