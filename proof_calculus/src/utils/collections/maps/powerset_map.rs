use std::collections::{BTreeMap, HashSet};
use std::hash::Hash;

use bitvec::bitbox;
use bitvec::boxed::BitBox;
use bitvec::vec::BitVec;
use itertools::Itertools;

use crate::utils::collections::maps::allocators::dense_usize_allocator::DenseUsizeAllocator;

/// Indexes in a PowerSetMap become invalidated whenever an item is removed from the PowerSetMap
#[derive(Clone, PartialEq, Eq, Debug, Hash, PartialOrd, Ord)]
pub struct PowerSetMapIndex(BitBox);
impl PowerSetMapIndex {
    fn new(vec: BitVec) -> Self {
        let mut index = vec.len();
        while index > 0 {
            index -= 1;
            if !vec[index] {
                break;
            }
        }
        Self(vec[0..index].into())
    }
}

// TODO: Consider improving efficiency of removal
/// A powerset map, maps every possible set over K to some V
///
/// Every key K that is inserted into the map is given some bit_id (usize)
/// Every set of keys S that is inserted into the map is given some index (PowerSetMapIndex)
/// Every PowerSetMapIndex is mapped to some value V
#[derive(Clone, PartialEq, Eq, Debug, Hash)]
pub struct PowerSetMap<K: Hash + Eq + Clone, V> {
    keys: DenseUsizeAllocator<usize, K>,
    values: BTreeMap<PowerSetMapIndex, V>,
}
impl<K: Hash + Eq + Clone, V> PowerSetMap<K, V> {
    fn index_to_keyset(&self, index: &PowerSetMapIndex) -> Result<HashSet<&K>, ()> {
        (0..index.0.len())
            .filter(|bit_id| index.0[*bit_id])
            .map(|bit_id| self.keys.get_value_by_key(&bit_id).ok_or(()))
            .try_collect()
    }
    fn keyset_to_index(&self, keyset: &HashSet<&K>) -> Result<PowerSetMapIndex, ()> {
        // Get bit_ids for all ids in hashset
        let bit_ids: Vec<_> = keyset
            .iter()
            .map(|key| match self.keys.get_key_by_value(key) {
                Some(v) => Ok(*v),
                None => Err(()),
            })
            .try_collect()?;
        // Construct index from bit_ids
        let mut index = {
            match bit_ids.iter().max() {
                Some(max) => bitbox!(0;max+1),
                None => bitbox!(0;0),
            }
        };
        for bit_id in bit_ids {
            index.set(bit_id, true);
        }
        // Convert to PowerSetMapIndex
        Ok(PowerSetMapIndex(index))
    }

    pub fn contains_key(&mut self, key: &K) -> bool {
        self.keys.contains_value(key)
    }
    pub fn insert_key(&mut self, key: K) {
        self.keys.insert_value(key);
    }
    pub fn remove_key(&mut self, key: &K) -> bool {
        // Remove the key from the key allocator
        if let Some(key_index) = self.keys.remove_by_value(key) {
            // Remove all values associated with that key
            self.values.retain(|index, _value| !index.0[key_index]);
            true
        } else {
            false
        }
    }
    pub fn extract_by_key(&mut self, key: &K) -> impl IntoIterator<Item = (HashSet<&K>, V)> {
        // Remove the key from the key allocator
        if let Some(bit_id_for_key) = self.keys.remove_by_value(key) {
            // Extract all values associated with that key
            let (indexes, mut results): (Vec<_>, Vec<_>) = self
                .values
                .extract_if(.., |index, _v| index.0[bit_id_for_key])
                .map(|(index, value)| (index, (HashSet::new(), value)))
                .unzip();

            // Construct the set of keys
            // Iterate through bit_ids, ignoring the one at key_index
            let bit_ids =
                (0..bit_id_for_key).chain(bit_id_for_key + 1..self.keys.allocated_keys_count());
            for bit_id in bit_ids {
                // Get the corresponding key for this bit_id
                if let Some(key) = self.keys.get_value_by_key(&bit_id) {
                    // Add the key to the hashmap of all indexes where the bit_id is true
                    for (index, (set, _value)) in indexes.iter().zip(results.iter_mut()) {
                        if index.0[bit_id] {
                            set.insert(key);
                        }
                    }
                }
            }
            results
        } else {
            vec![]
        }
    }
    
    pub fn get_value_by_keyset(&self, keyset: &HashSet<&K>) -> Option<&V> {
        let index = self.keyset_to_index(keyset).ok()?;
        self.values.get(&index)
    }
    pub fn get_value_mut_by_keyset(&mut self, keyset: &HashSet<&K>) -> Option<&mut V> {
        let index = self.keyset_to_index(keyset).ok()?;
        self.values.get_mut(&index)
    }
    pub fn insert_value_by_keyset(&mut self, keyset: &HashSet<&K>, value: V) -> Result<Option<V>,()> {
        let index = self.keyset_to_index(keyset)?;
        Ok(self.values.insert(index, value))
    }
}
