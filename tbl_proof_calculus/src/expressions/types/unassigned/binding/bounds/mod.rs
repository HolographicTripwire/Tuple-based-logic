mod expr_location;
mod variable_location;
mod variable_value;

mod equivalence;
mod identity;
mod insertion;

use crate::expressions::types::assigned::binding::bounds::{
    TblExpressionBoundAtomExactValue, TblExpressionBoundAtomExistsAtLocation,
    TblExpressionBoundCompoundExactLength, TblExpressionBoundCompoundExistsAtLocation,
    TblExpressionBoundValueDuplicated,
};

pub type UnassignedTblExpressionBoundAtomExistsAtLocation = TblExpressionBoundAtomExistsAtLocation;
pub type UnassignedTblPropositionBoundAtomExistsAtLocation =
    UnassignedTblExpressionBoundAtomExistsAtLocation;
pub type UnassignedTblExpressionBoundAtomExactValue = TblExpressionBoundAtomExactValue;
pub type UnassignedTblPropositionBoundAtomExactValue = UnassignedTblExpressionBoundAtomExactValue;
pub type UnassignedTblExpressionBoundCompoundExistsAtLocation =
    TblExpressionBoundCompoundExistsAtLocation;
pub type UnassignedTblPropositionBoundCompoundExistsAtLocation =
    UnassignedTblExpressionBoundCompoundExistsAtLocation;
pub type UnassignedTblExpressionBoundCompoundExactLength = TblExpressionBoundCompoundExactLength;
pub type UnassignedTblPropositionBoundCompoundExactLength =
    UnassignedTblExpressionBoundCompoundExactLength;
pub use expr_location::{
    UnassignedTblExpressionBoundExpressionExistsAtLocation,
    UnassignedTblPropositionBoundExpressionExistsAtLocation,
};
pub use variable_location::{
    UnassignedTblExpressionBoundVariableExistsAtLocation,
    UnassignedTblPropositionBoundVariableExistsAtLocation,
};
pub use variable_value::{
    UnassignedTblExpressionBoundVariableExactValue, UnassignedTblPropositionBoundVariableExactValue,
};
pub type UnassignedTblExpressionBoundValueDuplicated = TblExpressionBoundValueDuplicated;
pub type UnassignedTblPropositionBoundValueDuplicated = UnassignedTblExpressionBoundValueDuplicated;

pub use equivalence::{TblPropositionEquivalenceBound, UnassignedTblExpressionEquivalenceBound};
pub use identity::{UnassignedTblExpressionIdentityBound, UnassignedTblPropositionIdentityBound};
pub use insertion::{UnassignedTblExpressionInsertionBound, UnassignedTblPropositionSubsumedBound};
