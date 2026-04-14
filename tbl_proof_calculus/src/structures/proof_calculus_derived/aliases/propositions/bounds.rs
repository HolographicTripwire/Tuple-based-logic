use proof_calculus::structures::propositions::bounds::{PropositionBound, PropositionIdentityBounds};
use trait_aliases::trait_aliases;

use crate::structures::expressions::{TblExpression, compound::CompoundTblExpression};

trait_aliases!{
    pub trait TblPropositionIdentityBounds<'a, C: CompoundTblExpression, B> = PropositionIdentityBounds<'a, TblExpression<C>, B>;
}
