use crate::propositions::Expression;

#[derive(Clone)]
pub struct SubexpressionPath(Box<[usize]>);
impl <T: Into<Box<[usize]>>> From<T> for SubexpressionPath {
    fn from(value: T) -> Self { SubexpressionPath(value.into()) }
}
impl SubexpressionPath {
    pub fn join<'a>(&self, other: impl Into<&'a SubexpressionPath>) -> Self
        { Self([self.0.clone(),other.into().0.clone()].concat().into()) }
}
