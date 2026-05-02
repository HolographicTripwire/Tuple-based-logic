pub trait Combine: Sized {
    fn combine<I: IntoIterator<Item = Self>>(assignments: I) -> Self;
}
pub trait TryCombine: Sized {
    type CombinationError;
    fn combine<I: IntoIterator<Item = Self>>(assignments: I) -> Result<Self,Self::CombinationError>;
}
