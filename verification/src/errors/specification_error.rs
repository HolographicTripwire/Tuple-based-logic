use std::fmt::Display;

use dyn_clone::DynClone;
use path_lib::{obj_at_path::OwnedObjAtPath, paths::PathSeries};
use tbl_structures::{inference::{Inference, InferenceRule}, proof::{AtomicSubproofPath, OwnedPropositionInProof, OwnedSubexpressionInProof, OwnedSubproofInProof, Proof, ProofStep, PropositionInProof, SubexpressionInProof, SubproofInProof}};

use crate::{errors::validation_error::ProofValidationError, inference_rules::InferenceVerifier};

pub trait NaryPredicate<const n: usize,T>: 'static + DynClone + Fn([T; n]) -> bool {}
impl <P: 'static + Clone +  Fn([T; n]) -> bool, const n: usize,T> NaryPredicate<n,T> for P {}
impl <const n: usize, T> Clone for Box<dyn NaryPredicate<n,T>>
    { fn clone(&self) -> Self { dyn_clone::clone_box(&**self) } }

pub trait NaryStringifier<const n: usize,T>: 'static + DynClone + Fn([T; n]) -> String {}
impl <S: 'static + Clone +  Fn([T; n]) -> String, const n: usize,T> NaryStringifier<n,T> for S {}
impl <const n: usize, T> Clone for Box<dyn NaryStringifier<n,T>>
    { fn clone(&self) -> Self { dyn_clone::clone_box(&**self) } }

pub struct StringifiablePredicate<const n: usize, T> {
    predicate: Box<dyn NaryPredicate<n,T>>,
    stringifier: Box<dyn NaryStringifier<n,T>>
}
impl <const n: usize, T:'static + Clone> StringifiablePredicate<n,T> {
    pub fn new<Predicate: NaryPredicate<n,T>, Stringifier: NaryStringifier<n,T>>(predicate: Predicate, stringifier: Stringifier) -> Self 
        { Self{predicate: Box::new(predicate), stringifier: Box::new(stringifier)} }
    pub fn evaluate(&self, values: [T;n]) -> Result<(),StringifiableAssignment<n,T>> {
        if (self.predicate)(values.clone()) { Ok(()) }
        else { Err(StringifiableAssignment::new(Box::new(values), self.stringifier.clone())) }
    }
}

#[derive(Clone)]
pub struct StringifiableAssignment<const n: usize, T> {
    values: Box<[T;n]>,
    stringifier: Box<dyn NaryStringifier<n,T>>
}
impl <const n: usize, T: Clone> StringifiableAssignment<n,T> {
    fn new<Stringifier: NaryStringifier<n,T>>(values: Box<[T;n]>, stringifier: Stringifier) -> Self
        { Self{values: values, stringifier: Box::new(stringifier)} }
}
impl <const n: usize, T: Clone> Display for StringifiableAssignment<n,T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", (self.stringifier)(*self.values.clone()))
    }
}

impl <const n: usize, Rule: InferenceRule> ProofStepSpecificationErrorInner for StringifiableAssignment<n,OwnedSubproofInProof<Rule>>{}
impl <const n: usize> ProofStepSpecificationErrorInner for StringifiableAssignment<n,OwnedPropositionInProof>{}
impl <const n: usize> ProofStepSpecificationErrorInner for StringifiableAssignment<n,OwnedSubexpressionInProof>{}

trait ProofStepSpecificationErrorInner: 'static + Display + DynClone {}
dyn_clone::clone_trait_object!(ProofStepSpecificationErrorInner);

#[derive(Clone)]
pub struct ProofStepSpecificationError(Box<dyn ProofStepSpecificationErrorInner>);
impl ProofStepSpecificationError {
    pub fn from_inner<Inner: ProofStepSpecificationErrorInner>(inner: Inner) -> Self
        { Self(Box::new(inner)) }
}

pub fn verify_inference<Rule: VerifiableInferenceRule>(inference: &Inference<Rule>) -> Result<(),ProofValidationError> {
    let verifier = Rule::get_verifier(&inference.inference_type);
    match verifier(&inference) {
        Ok(()) => Ok(()),
        Err(err) => Err(ProofValidationError::InvalidStepSpecification(err)),
    }
}

pub trait VerifiableInferenceRule: InferenceRule {
    fn get_verifier(rule: &Self) -> impl InferenceVerifier<Self>;
}
