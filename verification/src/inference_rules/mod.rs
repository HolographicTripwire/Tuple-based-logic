mod deduction;
mod verbatim;

use deduction::*;
use verbatim::*;

use tbl_structures::{inference::{Inference, InferenceRule}};

use crate::errors::specification_error::VerifiableInferenceRule;


#[derive(Clone,PartialEq)]
pub enum StandardInferenceRule {
    // Deduction rules
    ConjunctionIntroduction,
    ImplicationElimination,
    UniversalSubstitution,
    // Verbatim rules
    AtomicityAssertion,
    AtomDifferentiation,
    TupleAppendation,
}
#[derive(Clone)]
pub enum StandardInferenceErr {
    ConjunctionIntroduction(ConjunctionIntroductionError),
    ImplicationElimination(ImplicationEliminationError),
    UniversalSubstitution(UniversalSubstitutionError),
    AtomicityAssertion(AtomicityAssertionError),
    AtomDifferentiation(AtomDifferentiationError),
    TupleAppendation(TupleAppendationError),
}

impl InferenceRule for StandardInferenceRule {}
impl VerifiableInferenceRule<StandardInferenceErr> for StandardInferenceRule {
    fn verify(inference: &Inference<Self>) -> Result<(),StandardInferenceErr> {
        match inference.0.obj().inference_type {
            StandardInferenceRule::ConjunctionIntroduction => verify_conjunction_introduction(inference)
                .map_err(|e| StandardInferenceErr::ConjunctionIntroduction(e)),
            StandardInferenceRule::ImplicationElimination => verify_implication_elimination(inference)
                .map_err(|e| StandardInferenceErr::ImplicationElimination(e)),
            StandardInferenceRule::UniversalSubstitution => verify_universal_substitution(inference)
                .map_err(|e| StandardInferenceErr::UniversalSubstitution(e)),
            StandardInferenceRule::AtomicityAssertion => verify_atomicity_assertion(inference)
                .map_err(|e| StandardInferenceErr::AtomicityAssertion(e)),
            StandardInferenceRule::AtomDifferentiation => verify_atom_differentiation(inference)
                .map_err(|e| StandardInferenceErr::AtomDifferentiation(e)),
            StandardInferenceRule::TupleAppendation => verify_tuple_appendation(inference)
                .map_err(|e| StandardInferenceErr::TupleAppendation(e)),
        }
    }
}
