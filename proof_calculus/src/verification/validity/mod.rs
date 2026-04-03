use std::marker::PhantomData;

use itertools::Itertools;
use path_lib::obj_at_path::{ObjAtPath, OwnedObjAtPath};

use crate::{structures::{inferences::{Inference, InferenceRule, located::InferenceAtPath}, propositions::Proposition, sequential_proofs::{SequentialProof, subproofs::{SequentialProofAtPath, SequentialProofInProofPath, immediate::ImmediateSequentialProofInProofPath}}}, verification::validity::{error::{OwnedProofValidityErrorAtPath, ProofValidityError}, stepper::{ProofValidityStepErr, ProofValidityStepResult, ProofValidityStepper}}};

pub mod inferences;
pub mod abstract_proofs;
pub mod sequential_proofs;
pub mod assertions;
pub mod error;
pub mod stepper;

pub trait ValidatableInferenceRule<P:Proposition>: InferenceRule<P> {
    type Err: Clone;

    fn validate(inference: &Inference<P,Self>) -> Result<(),Self::Err>;
    //fn validate_located<Path>(located_inference: InferenceAtPath<P,Self,Path>)
}


pub fn validate_inference<P: Proposition,Rule: ValidatableInferenceRule<P>>(inference: &Inference<P,Rule>) -> Result<(),ProofValidityError<P,Rule::Err>> {
    Rule::validate(inference)
        .map_err(|err| ProofValidityError::InvalidInference(err,PhantomData))
}
pub fn validate_located_inference<'a, P: Proposition, Rule: ValidatableInferenceRule<P>,Path>(inference: InferenceAtPath<'a,P,Rule,Path>) -> Result<(),OwnedProofValidityErrorAtPath<P,Rule::Err,Path>> {
    validate_inference(inference.obj).map_err(|err| OwnedObjAtPath{obj: err, path: inference.path})
}

pub fn verify_proof_validity<'a, P: Proposition, Rule: ValidatableInferenceRule<P>>(proof: &'a SequentialProof<P,Rule>) -> Result<(),ProofValidityStepErr<P,Rule::Err,(),SequentialProofInProofPath>> {
    get_proof_validity_errors(proof).try_collect()
}


pub fn get_proof_validity_errors<'a, P: Proposition, Rule: ValidatableInferenceRule<P>>(proof: &'a SequentialProof<P,Rule>) -> impl Iterator<Item = ProofValidityStepResult<P,Rule::Err,(),SequentialProofInProofPath>> {
    get_located_proof_validity_errors(ObjAtPath { obj: proof, path: SequentialProofInProofPath(vec![]) })
        .map(|result: ProofValidityStepResult<P, <Rule as ValidatableInferenceRule<P>>::Err, _, SequentialProofInProofPath>|
            result.map_err(|err| err.replace_path(|_| (), |x| x))
        )
}
pub fn get_located_proof_validity_errors<'a, P: Proposition, Rule: ValidatableInferenceRule<P>,ParentPath:'a + Clone,JoinedPath: 'a + Clone + From<(ParentPath,ImmediateSequentialProofInProofPath)> + From<(JoinedPath,ImmediateSequentialProofInProofPath)>>(proof: SequentialProofAtPath<'a,P,Rule,ParentPath>) -> impl Iterator<Item = ProofValidityStepResult<P,Rule::Err,ParentPath,JoinedPath>> {
    ProofValidityStepper::new(proof)
}
