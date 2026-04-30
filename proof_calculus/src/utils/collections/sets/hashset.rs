use std::hash::Hash;
use std::collections::HashSet;

pub fn transform_hashset<T1,T2:Hash + Eq,F: Fn(T1) -> T2>(set: HashSet<T1>, f: F) -> HashSet<T2>
    { set.into_iter().map(f).collect() }
