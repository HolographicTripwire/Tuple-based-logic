mod proof_step;
mod expression;
mod proposition;
mod stepper;

use path_lib::paths::PathSeries;
pub use proof_step::*;
pub use expression::*;
pub use proposition::*;


use tbl_structures::{expressions::TblPropSet, proof::{Proof, ProofInProof, error::OwnedErrorInProof, inference::{Inference, InferenceRule}}};

pub fn verify_inference<Err, Rule: VerifiableInferenceRule<Err>>(inference: &Inference<Rule>) -> Result<(),ProofValidityError<Err>> {
    Rule::verify(inference)
        .map_err(|err| ProofValidityError::InvalidInference(err))
}



#[derive(Clone)]
pub enum ProofValidityError<InferenceErr> {
    AssumptionsNotFound(TblPropSet),
    ConclusionsNotFound(TblPropSet),
    InvalidInference(InferenceErr),
}

pub fn verify_proof_validity<'a, E: Clone, Rule: VerifiableInferenceRule<E>>(proof: &'a Proof<Rule>) -> Result<(),OwnedErrorInProof<ProofValidityError<E>>> {
    match get_proof_validity_errors(proof).next() {
        Some(err) => Err(err),
        None => Ok(()),
    }
}
pub fn get_proof_validity_errors<'a, E: Clone, Rule: VerifiableInferenceRule<E>>(proof: &'a Proof<Rule>) -> impl Iterator<Item = OwnedErrorInProof<ProofValidityError<E>>> {
    proof_validity_helper(&ProofInProof::from_inner(proof,PathSeries::empty()))
}
