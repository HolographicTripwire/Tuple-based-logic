mod deduction;
mod verbatim;

use deduction::*;
use verbatim::*;

use tbl_structures::{inference::InferenceRule, proof::{InferenceInProof}};

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
pub enum StandardInferenceErr {

}

impl InferenceRule for StandardInferenceRule {}
impl VerifiableInferenceRule<StandardInferenceErr> for StandardInferenceRule {
    fn verify(rule: &InferenceInProof<Self>) -> Result<(),StandardInferenceErr> {
        match rule.0.obj().inference_type {
            StandardInferenceRule::ConjunctionIntroduction => todo!(),
            StandardInferenceRule::ImplicationElimination => todo!(),
            StandardInferenceRule::UniversalSubstitution => todo!(),
            StandardInferenceRule::AtomicityAssertion => todo!(),
            StandardInferenceRule::AtomDifferentiation => todo!(),
            StandardInferenceRule::TupleAppendation => todo!(),
        }
    }
}
