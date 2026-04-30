use proof_calculus::propositions::assignments::{PartialPropositionalAssignment, PropositionalAssignment};
use trait_aliases::trait_aliases;

use crate::{expressions::types::unassigned::compound::UnassignedCompoundTblExpression, proof_calculus_derived::aliases::propositions::UnassignedTblProposition};

trait_aliases!{
    pub trait TblPropositionalAssignment<C: UnassignedCompoundTblExpression> = PropositionalAssignment<UnassignedTblProposition<C>>;
    pub trait PartialTblPropositionalAssignment<C: UnassignedCompoundTblExpression> = PartialPropositionalAssignment<UnassignedTblProposition<C>>;
}
