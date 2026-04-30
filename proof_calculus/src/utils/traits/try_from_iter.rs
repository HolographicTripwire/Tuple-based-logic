pub trait TryFromIterator<A>: Sized {
    type Error;

    fn try_from_iter<T>(iter: T) -> Result<Self, Self::Error>
       where T: Iterator<Item = A>;
}
