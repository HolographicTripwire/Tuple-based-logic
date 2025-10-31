mod deduction;
mod verbatim;

use deduction::*;
use tbl_textualization::structures::expressions::ExpressionStyle;
use verbatim::*;

use tbl_structures::{inference::InferenceRule, proof::OwnedInferenceInProof};

use crate::errors::specification_error::{ProofStepSpecificationError, VerifiableInferenceRule};

/// A function that checks if a Proof step is valid, and if not returns an error of type E
pub trait InferenceVerifier<'a, Rule: VerifiableInferenceRule>: Fn(&'a OwnedInferenceInProof<Rule>, ExpressionStyle<'a>) -> Result<(),ProofStepSpecificationError<'a>> {}
impl <'a, Rule: VerifiableInferenceRule, F: Fn(&'a OwnedInferenceInProof<Rule>, ExpressionStyle<'a>) -> Result<(),ProofStepSpecificationError<'a>>> InferenceVerifier<'a,Rule> for F {}

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
impl InferenceRule for StandardInferenceRule {}
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
