use std::marker::PhantomData;

use path_lib::obj_at_path::{ObjAtPath, OwnedObjAtPath};

use crate::{proofs::sequential::errors::{ErrorInProof, OwnedErrorInProof}, propositions::types::assigned::{Proposition, collections::sets::implementations::hash::HashPropSet1O}};

#[derive(Clone,PartialEq,Eq,Debug)]
pub enum ProofValidityError<P:Proposition, InferenceErr> {
    AssumptionsNotFound(HashPropSet1O<P>),
    ConclusionsNotFound(HashPropSet1O<P>),
    InvalidInference(InferenceErr,PhantomData<P>),
}

pub type ProofValidityErrorAtPath<'a,P:Proposition,InferenceErr,Path> = ObjAtPath<'a,ProofValidityError<P,InferenceErr>,Path>;
pub type OwnedProofValidityErrorAtPath<P:Proposition,InferenceErr,Path> = OwnedObjAtPath<ProofValidityError<P,InferenceErr>,Path>;

pub type ValidityErrorInProof<'a,P:Proposition,InferenceErr> = ErrorInProof<'a,ProofValidityError<P,InferenceErr>>;
pub type OwnedValidityErrorInProof<P:Proposition,InferenceErr> = OwnedErrorInProof<ProofValidityError<P,InferenceErr>>;
