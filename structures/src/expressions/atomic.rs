use std::num::TryFromIntError;

use enum_iterator::Sequence;

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

/// Atoms which are built in to Tuple-Based Logic, and will appear in all axiomatic systems in Tuple-Based Logic
#[derive(Clone,Sequence)]
pub enum BuiltInAtom {
    // Deduction
    Conjunction,
    Implication,
    UniversalQuantifier,
    // Identity
    Identity,
    // Contradiction
    Negation,
    // Verbatim
    Verbatim,
    Concatenate,
    Atomic,
}

impl Into<AtomicExpression> for BuiltInAtom {
    /// Assigns each built in atom a unique atom id
    fn into(self) -> AtomicExpression {
        AtomicExpression(match self {
            // Deduction
            BuiltInAtom::Conjunction => 0,
            BuiltInAtom::UniversalQuantifier => 1,
            BuiltInAtom::Implication => 2,
            // Contradiction
            BuiltInAtom::Negation => 3,
            // Identity
            BuiltInAtom::Identity => 4,
            // Verbatim
            BuiltInAtom::Verbatim => 5,
            BuiltInAtom::Concatenate => 6,
            BuiltInAtom::Atomic => 7,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use enum_iterator::all;

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

    #[test]
    fn test_differentiation_of_builtins() {
        let builtins  = all::<BuiltInAtom>().collect::<Vec<_>>();
        for (i, ix) in builtins.iter().enumerate() {
            for (j, jx) in builtins.iter().enumerate() {
                let ia: AtomicExpression = (*ix).clone().into();
                let ja: AtomicExpression = (*jx).clone().into();
                if i==j { assert_eq!(ia,ja) }
                else { assert_ne!(ia,ja) }
            }
        }
    }
}
