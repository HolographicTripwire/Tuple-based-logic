use proof_calculus::propositions::assignments::{PartialPropositionalAssignment, PropositionalAssignment};
use trait_aliases::trait_aliases;

use crate::{expressions::types::unassigned::compound::UnassignedTblExpressionCompound, proof_calculus_derived::aliases::propositions::types::UnassignedTblProposition};

trait_aliases!{
    pub trait TblPropositionalAssignment<C: UnassignedTblExpressionCompound> = PropositionalAssignment<UnassignedTblProposition<C>>;
    pub trait PartialTblPropositionalAssignment<C: UnassignedTblExpressionCompound> = PartialPropositionalAssignment<UnassignedTblProposition<C>>;
}
