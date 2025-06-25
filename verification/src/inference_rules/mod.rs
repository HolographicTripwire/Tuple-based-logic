pub mod assertions;
pub mod error;
mod deduction;
mod verbatim;

use deduction::*;
use verbatim::*;

use tbl_structures::inference::Inference;

use crate::{inference_rules::error::{ProofStepSpecificationError, VerifiableInferenceRule}};

/// A function that checks if a Proof step is valid, and if not returns an error of type E
trait InferenceVerifier<Rule: VerifiableInferenceRule>: Fn(&Inference<Rule>) -> Result<(),ProofStepSpecificationError> {}
impl <Rule: VerifiableInferenceRule, F: Fn(&Inference<Rule>) -> Result<(),ProofStepSpecificationError>> InferenceVerifier<Rule> for F {}

#[derive(Clone)]
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
impl VerifiableInferenceRule for StandardInferenceRule {
    fn get_verifier(rule: &Self) -> impl InferenceVerifier<Self> {
        match rule {
            // Deduction rules
            StandardInferenceRule::ConjunctionIntroduction => verify_conjunction_introduction,
            StandardInferenceRule::ImplicationElimination => verify_implication_elimination,
            StandardInferenceRule::UniversalSubstitution => verify_universal_substitution,
            // Verbatim rules
            StandardInferenceRule::AtomicityAssertion => verify_atomicity_assertion,
            StandardInferenceRule::AtomDifferentiation => verify_atom_differentiation,
            StandardInferenceRule::TupleAppendation => verify_tuple_appendation,
        }
    }
}
