pub mod optional_iterator;

pub fn split_into_max_by_key<T, K, I, F>(iter: I, mut key: F) -> Option<(T, Vec<T>)>
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
