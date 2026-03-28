use std::num::TryFromIntError;

/// An [Identifier] used for Atom objects, which are used for building tuple objects in Tuple-based logic
#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
pub struct AtomicTblExpression(pub u16);
impl AtomicTblExpression {
    pub fn first() -> AtomicTblExpression { AtomicTblExpression(0) }
    pub fn next(&self) -> AtomicTblExpression { AtomicTblExpression(self.0 + 1) }
}
impl TryFrom<usize> for AtomicTblExpression {
    type Error = TryFromIntError;
    fn try_from(value: usize) -> Result<Self, Self::Error>
        { match u16::try_from(value) {
            Ok(val) => Ok(Self(val)),
            Err(err) => Err(err),
        }}
}
impl TryFrom<AtomicTblExpression> for usize {
    type Error = TryFromIntError;
    fn try_from(value: AtomicTblExpression) -> Result<Self, Self::Error>
        { Ok(usize::try_from(value.0)?) }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_id_initialisation() {
        let result = AtomicTblExpression::first();
        assert_eq!(result, AtomicTblExpression(0));
    }

    #[test]
    #[should_panic]
    fn test_id_overflow() {
        let result = AtomicTblExpression(u16::max_value());
        result.next();
    }
}
