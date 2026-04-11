use std::collections::HashSet;

use nom::Map;

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
    pub fn merge_without_conflicts<I: IntoIterator<Item=Self>>(maps: I) -> Result<Self,(usize,V,V)> where V: PartialEq<V> {
        // Use the largest map as a starting point
        let (mut largest, remaining) = match split_into_max_by_key(maps, |m| m.values.len()) {
            Some(v) => v,
            None => return Ok(DenseUsizeMap::default()),
        };
        // Fill the largest map with values from the smaller maps
        for map in remaining {
            for (key, value1) in map {
                // Insert the value
                match std::mem::replace(largest.values.get_mut(key).expect(KEYSET_VALUE_OUT_OF_BOUNDS_EXCEPTION), Some(value1)) {
                    // If there was already a value there
                    Some(old_value) => { 
                        // A value was just inserted, so panic if it's no longer there
                        let new_value = match largest.values.get_mut(key) {
                            Some(v) => v,
                            _ => panic!("{}",UNASSIGNED_JUST_INSERTED_VALUE_EXCEPTION)
                        };
                        assert!(new_value.is_some(), "{}", UNASSIGNED_JUST_INSERTED_VALUE_EXCEPTION);
                        // Check if the value is the same as it was before, and return Err() containing the conflict otherwise
                        if let Some(new_value) = new_value.take_if(|v| v == &old_value) {
                            return Err((key,old_value,new_value));
                        }

                    }, None => { largest.assigned.insert(key); }
                }
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
            }).collect::<Vec<_>>().into_iter()
    }
}

impl <const N: usize, V> From<[(usize,V); N]> for DenseUsizeMap<V> {
    fn from(pairs: [(usize,V); N]) -> Self {
        Self::from(Vec::from(pairs))
    }
}
impl <V> From<Vec<(usize,V)>> for DenseUsizeMap<V> {
    fn from(pairs: Vec<(usize,V)>) -> Self {
        match pairs.iter().map(|(key,_value)| key).max() {
            Some(v) => {
                let mut values: Vec<Option<V>> = (0..*v).map(|_| None).collect();
                let assigned = pairs.iter().map(|(key,_value)| *key).collect();
                for (k,v) in pairs { values[k] = Some(v) }
                Self{assigned, values}
            },
            None => Self::default(),
        }
    }
}

fn split_into_max_by_key<T, K, I, F>(iter: I, mut key: F) -> Option<(T, Vec<T>)>
where
    I: IntoIterator<Item = T>,
    F: FnMut(&T) -> K,
    K: Ord,
{
    let mut it = iter.into_iter();
    let first = it.next()?;
    let mut max = first;
    let mut max_key = key(&max);
    let mut others = Vec::new();

    for item in it {
        let k = key(&item);
        if k > max_key {
            others.push(std::mem::replace(&mut max, item));
            max_key = k;
        } else {
            others.push(item);
        }
    }
    Some((max, others))
}
