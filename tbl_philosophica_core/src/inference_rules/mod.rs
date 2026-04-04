mod deduction;
mod verbatim;

use deduction::*;
use proof_calculus::{structures::inferences::InferenceRule, verification::validity::ValidatableInferenceRule};
use tbl_proof_calculus::structures::{expressions::compound::CompoundTblExpression, proof_calculus_derived::aliases::{inferences::{TblInference}, propositions::TblProposition}};
use verbatim::*;

#[derive(Clone,PartialEq)]
pub enum PhilosophicaInferenceRule {
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
pub enum PhilosophicaInferenceErr<C: CompoundTblExpression> {
    ConjunctionIntroduction(ConjunctionIntroductionError<C>),
    ImplicationElimination(ImplicationEliminationError<C>),
    UniversalSubstitution(UniversalSubstitutionError<C>),
    AtomicityAssertion(AtomicityAssertionError<C>),
    AtomDifferentiation(AtomDifferentiationError<C>),
    TupleAppendation(TupleAppendationError<C>),
}

impl <C: CompoundTblExpression> InferenceRule<TblProposition<C>> for PhilosophicaInferenceRule {}
impl <C: CompoundTblExpression> ValidatableInferenceRule<TblProposition<C>> for PhilosophicaInferenceRule {
    type Err = PhilosophicaInferenceErr<C>;
    fn validate(inference: &TblInference<C,Self>) -> Result<(),PhilosophicaInferenceErr<C>> {
        match inference.inference_type {
            PhilosophicaInferenceRule::ConjunctionIntroduction => validate_conjunction_introduction(inference)
                .map_err(|e| PhilosophicaInferenceErr::ConjunctionIntroduction(e)),
            PhilosophicaInferenceRule::ImplicationElimination => verify_implication_elimination(inference)
                .map_err(|e| PhilosophicaInferenceErr::ImplicationElimination(e)),
            PhilosophicaInferenceRule::UniversalSubstitution => verify_universal_substitution(inference)
                .map_err(|e| PhilosophicaInferenceErr::UniversalSubstitution(e)),
            PhilosophicaInferenceRule::AtomicityAssertion => verify_atomicity_assertion(inference)
                .map_err(|e| PhilosophicaInferenceErr::AtomicityAssertion(e)),
            PhilosophicaInferenceRule::AtomDifferentiation => verify_atom_differentiation(inference)
                .map_err(|e| PhilosophicaInferenceErr::AtomDifferentiation(e)),
            PhilosophicaInferenceRule::TupleAppendation => verify_tuple_appendation(inference)
                .map_err(|e| PhilosophicaInferenceErr::TupleAppendation(e)),
        }
    }
}
