mod assumption_count;
mod explicit_conclusion_count;

pub use assumption_count::*;

pub fn as_slice<I: IntoIterator>(into_iter: I) -> Box<[I::Item]>
    { into_iter.into_iter().collect() }
pub fn as_sized_slice<const N: usize,I: IntoIterator>(into_iter: I) -> Result<[<I as IntoIterator>::Item;N],Vec<<I as IntoIterator>::Item>> { 
    into_iter
        .into_iter()
        .collect::<Vec<_>>()
        .try_into()
}
