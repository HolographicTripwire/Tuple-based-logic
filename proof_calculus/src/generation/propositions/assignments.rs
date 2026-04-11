pub trait PropositionalAssignment: Sized {
    
    fn combine<I: IntoIterator<Item = Self>>(assignments: I) -> Result<Self,()>;
}
pub trait PartialPropositionalAssignment: Sized {
    fn combine<I: IntoIterator<Item = Self>>(assignments: I) -> Result<Self,()>;
}
