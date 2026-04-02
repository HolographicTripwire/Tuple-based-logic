use proof_calculus::verification::ValidatableInferenceRule;
use trait_aliases::trait_aliases;

use crate::structures::{expressions::compound::CompoundTblExpression, proof_calculus_derived::aliases::propositions::TblProposition};

trait_aliases!{
    pub trait ValidatableTblInferenceRule<C:CompoundTblExpression> = ValidatableInferenceRule<TblProposition<C>>;
}
