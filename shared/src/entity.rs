use std::num::TryFromIntError;

use ids::{Id16, IdImpl, Identifier};

/// Entities which are built in to Tuple-Based Logic, and will appear in all axiomatic systems in Tuple-Based Logic
pub enum BuiltinEntity {
    Conjunction,
    Implication,
    UniversalQuantifier,
    Identity,
    TupleAppend,
}
impl Into<EntityId> for BuiltinEntity {
    fn into(self) -> EntityId {
        let id = match self {
            BuiltinEntity::Conjunction => 0,
            BuiltinEntity::Implication => 1,
            BuiltinEntity::UniversalQuantifier => 2,
            BuiltinEntity::Identity => 3,
            BuiltinEntity::TupleAppend => 4,
        };
        EntityId(Id16(id))
    }
}

/// An [Identifier] used for Entity objects
#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
pub struct EntityId(Id16);
impl Identifier for EntityId {
    fn first() -> Self { Self(Id16::first()) }
    fn next(self) -> Self { Self(self.0.next().expect("Out of term ids")) }
}
impl TryFrom<usize> for EntityId {
    type Error = TryFromIntError;
    fn try_from(value: usize) -> Result<Self, Self::Error>
        { Ok(EntityId(Id16::try_from(value)?)) }
}
impl TryFrom<EntityId> for usize {
    type Error = TryFromIntError;
    fn try_from(value: EntityId) -> Result<Self, Self::Error>
        { Ok(usize::try_from(value.0)?) }
}
