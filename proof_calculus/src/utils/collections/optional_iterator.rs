pub enum OptionalIterator<I: Iterator> {
    Some(I), None
}
impl <I: Iterator> Iterator for OptionalIterator<I> {
    type Item = I::Item;
    fn next(&mut self) -> Option<Self::Item> { match self {
        OptionalIterator::Some(iter) => iter.next(),
        OptionalIterator::None => None,
    }}
}
