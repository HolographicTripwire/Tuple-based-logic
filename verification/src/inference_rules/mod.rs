mod deduction;
mod verbatim;

use deduction::*;
use tbl_textualization::structures::expressions::ExpressionStyle;
use verbatim::*;

use tbl_structures::{inference::InferenceRule, proof::{InferenceInProof}};

use crate::errors::specification_error::{ProofStepSpecificationError, VerifiableInferenceRule};

/// A function that checks if a Proof step is valid, and if not returns an error of type E
pub trait InferenceVerifier<Rule: VerifiableInferenceRule>: for <'a> Fn(&InferenceInProof<Rule>, ExpressionStyle<'a>) -> Result<(),ProofStepSpecificationError<'a>> {}
impl <Rule: VerifiableInferenceRule, F: for<'a> Fn(&InferenceInProof<Rule>, ExpressionStyle<'a>) -> Result<(),ProofStepSpecificationError<'a>>> InferenceVerifier<Rule> for F {}

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
