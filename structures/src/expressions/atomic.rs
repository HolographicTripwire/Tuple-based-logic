use std::num::TryFromIntError;

/// An [Identifier] used for Atom objects, which are used for building tuple objects in Tuple-based logic
#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
pub struct AtomicExpression(pub u16);
impl AtomicExpression {
    pub fn first() -> AtomicExpression { AtomicExpression(0) }
    pub fn next(&self) -> AtomicExpression { AtomicExpression(self.0 + 1) }
}
impl TryFrom<usize> for AtomicExpression {
    type Error = TryFromIntError;
    fn try_from(value: usize) -> Result<Self, Self::Error>
        { match u16::try_from(value) {
            Ok(val) => Ok(Self(val)),
            Err(err) => Err(err),
        }}
}
impl TryFrom<AtomicExpression> for usize {
    type Error = TryFromIntError;
    fn try_from(value: AtomicExpression) -> Result<Self, Self::Error>
        { Ok(usize::try_from(value.0)?) }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_id_initialisation() {
        let result = AtomicExpression::first();
        assert_eq!(result, AtomicExpression(0));
    }

    #[test]
    #[should_panic]
    fn test_id_overflow() {
        let result = AtomicExpression(u16::max_value());
        result.next();
    }
}
