pub trait Combine: Sized {
    fn combine<I: IntoIterator<Item = Self>>(to_combine: I) -> Self;
}
pub trait TryCombine: Sized {
    type CombinationError;
    fn try_combine<I: IntoIterator<Item = Self>>(to_combine: I) -> Result<Self,Self::CombinationError>;
}
