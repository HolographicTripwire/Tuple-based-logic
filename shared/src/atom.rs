use std::num::TryFromIntError;

use ids::{Id16, IdImpl, Identifier};

/// Atoms which are built in to Tuple-Based Logic, and will appear in all axiomatic systems in Tuple-Based Logic
pub enum BuiltinAtom {
    Conjunction,
    Implication,
    UniversalQuantifier,
    Identity,
    TupleAppend,
}
impl Into<AtomId> for BuiltinAtom {
    fn into(self) -> AtomId {
        let id = match self {
            BuiltinAtom::Conjunction => 0,
            BuiltinAtom::Implication => 1,
            BuiltinAtom::UniversalQuantifier => 2,
            BuiltinAtom::Identity => 3,
            BuiltinAtom::TupleAppend => 4,
        };
        AtomId(Id16(id))
    }
}

/// An [Identifier] used for Atom objects
#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
pub struct AtomId(Id16);
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
