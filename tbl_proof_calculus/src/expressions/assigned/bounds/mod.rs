pub mod atom;
pub mod compound;
pub mod duplication;
pub mod iterators;

pub use atom::TblExpressionBoundAtomExactValue;
pub use compound::TblExpressionBoundCompoundExactLength;
pub use duplication::TblExpressionBoundValueDuplicated;

use crate::expressions::assigned::subexpressions::TblSubexpressionInExpressionPath;

pub enum TblExpressionIdentityBound {
    AtomValue(TblExpressionBoundAtomExactValue),
    CompoundLength(TblExpressionBoundCompoundExactLength),
}
impl TblExpressionIdentityBound {
    fn path(&self) -> &TblSubexpressionInExpressionPath { match self {
        TblExpressionIdentityBound::AtomValue(atom_bound) => &atom_bound.path,
        TblExpressionIdentityBound::CompoundLength(compound_bound) => &compound_bound.path,
    } }
}
impl From<TblExpressionBoundAtomExactValue> for TblExpressionIdentityBound {
    fn from(bound: TblExpressionBoundAtomExactValue) -> Self
        { Self::AtomValue(bound) }
}
impl From<TblExpressionBoundCompoundExactLength> for TblExpressionIdentityBound {
    fn from(bound: TblExpressionBoundCompoundExactLength) -> Self
        { Self::CompoundLength(bound) }
}

pub type TblPropositionIdentityBound = TblExpressionIdentityBound;
