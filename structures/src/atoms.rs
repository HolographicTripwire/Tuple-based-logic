use std::num::TryFromIntError;

use enum_iterator::Sequence;
use ids::{Id16, IdImpl, Identifier};

/// An [Identifier] used for Atom objects, which are used for building tuple objects in Tuple-based logic
#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
pub struct AtomId(pub Id16);

impl Identifier for AtomId {
    fn first() -> Self { Self(Id16::first()) }
    fn next(self) -> Self { Self(self.0.next().expect("Out of term ids")) }
}
impl TryFrom<usize> for AtomId {
    type Error = TryFromIntError;
    fn try_from(value: usize) -> Result<Self, Self::Error>
        { Ok(AtomId(Id16::try_from(value)?)) }
}
impl TryFrom<AtomId> for usize {
    type Error = TryFromIntError;
    fn try_from(value: AtomId) -> Result<Self, Self::Error>
        { Ok(usize::try_from(value.0)?) }
}

/// Atoms which are built in to Tuple-Based Logic, and will appear in all axiomatic systems in Tuple-Based Logic
#[derive(Clone,Sequence)]
pub enum BuiltInAtom {
    // Deduction
    Conjunction,
    Implication,
    UniversalQuantifier,
    // Contradiction
    Negation,
    // Identity
    Identity,
    NonIdentity,
    // Verbatim
    Verbatim,
    Atomic,
    TupleAppend,
}

impl Into<AtomId> for BuiltInAtom {
    /// Assigns each built in atom a unique atom id
    fn into(self) -> AtomId {
        let id = match self {
            // Deduction
            BuiltInAtom::Conjunction => 0,
            BuiltInAtom::Implication => 1,
            BuiltInAtom::UniversalQuantifier => 2,
            // Contradiction
            BuiltInAtom::Negation => 3,
            // Identity
            BuiltInAtom::Identity => 4,
            BuiltInAtom::NonIdentity => 5,
            // Verbatim
            BuiltInAtom::Verbatim => 6,
            BuiltInAtom::Atomic => 7,
            BuiltInAtom::TupleAppend => 8,
        };
        AtomId(Id16(id))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use enum_iterator::all;

    #[test]
    fn test_id_initialisation() {
        let result = AtomId::first();
        assert_eq!(result, AtomId(Id16(0)));
    }

    #[test]
    #[should_panic]
    fn test_id_overflow() {
        let result = AtomId(Id16(u16::max_value()));
        result.next();
    }

    #[test]
    fn test_differentiation_of_builtins() {
        let builtins  = all::<BuiltInAtom>().collect::<Vec<_>>();
        for (i, ix) in builtins.iter().enumerate() {
            for (j, jx) in builtins.iter().enumerate() {
                let ia: AtomId = (*ix).clone().into();
                let ja: AtomId = (*jx).clone().into();
                if i==j { assert_eq!(ia,ja) }
                else { assert_ne!(ia,ja) }
            }
        }
    }
}
