use proof_calculus::proofs::errors::ValidatableInferenceRule;
use trait_aliases::trait_aliases;

use crate::{expressions::types::assigned::compound::CompoundTblExpression, proof_calculus_derived::aliases::propositions::types::TblProposition};

trait_aliases!{
    pub trait ValidatableTblInferenceRule<C:CompoundTblExpression> = ValidatableInferenceRule<TblProposition<C>>;
}
