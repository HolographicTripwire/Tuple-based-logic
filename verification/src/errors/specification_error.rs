use std::fmt::Display;
use dyn_clone::DynClone;

use tbl_structures::{inference::{Inference, InferenceRule}, proof::{OwnedPropositionInProof, OwnedSubexpressionInProof, OwnedSubproofInProof}};

use crate::{errors::validation_error::ProofValidationError, inference_rules::InferenceVerifier};

pub trait NaryPredicate<'a,const n: usize,T>: 'a + DynClone + Fn([T; n]) -> bool {}
impl <'a,P: 'static + Clone +  Fn([T; n]) -> bool, const n: usize,T> NaryPredicate<'a,n,T> for P {}
impl <'a,const n: usize, T> Clone for Box<dyn NaryPredicate<'a,n,T>>
    { fn clone(&self) -> Self { dyn_clone::clone_box(&**self) } }

pub trait NaryStringifier<'a,const n: usize,T: 'a + Clone>: 'a + DynClone + Fn([T; n]) -> String {
    fn assign(&'a self, values: [T; n]) -> StringifiableAssignment<'a, n, T>
        { StringifiableAssignment::new(Box::new(values), self.clone()) } 
}
impl <'a,S: 'a + Clone +  Fn([T; n]) -> String, const n: usize,T: 'a + Clone> NaryStringifier<'a,n,T> for S {}
impl <'a,const n: usize, T> Clone for Box<dyn NaryStringifier<'a,n,T>>
    { fn clone(&self) -> Self { dyn_clone::clone_box(&**self) }}

pub struct StringifiablePredicate<'a,const n: usize, T> {
    predicate: Box<dyn NaryPredicate<'a,n,T>>,
    stringifier: Box<dyn NaryStringifier<'a,n,T>>
}
impl <'a,const n: usize, T:'a + Clone> StringifiablePredicate<'a,n,T> {
    pub fn new<Predicate: NaryPredicate<'a,n,T>, Stringifier: NaryStringifier<'a,n,T>>(predicate: Predicate, stringifier: Stringifier) -> Self 
        { Self{predicate: Box::new(predicate), stringifier: Box::new(stringifier)} }
    pub fn evaluate(&self, values: [T;n]) -> Result<(),StringifiableAssignment<'a,n,T>> {
        if (self.predicate)(values.clone()) { Ok(()) }
        else { Err(StringifiableAssignment::new(Box::new(values), self.stringifier.clone())) }
    }
}

#[derive(Clone)]
pub struct StringifiableAssignment<'a,const n: usize, T> {
    values: Box<[T;n]>,
    stringifier: Box<dyn NaryStringifier<'a,n,T>>
}
impl <'a,const n: usize, T: Clone> StringifiableAssignment<'a,n,T> {
    fn new<Stringifier: NaryStringifier<'a,n,T>>(values: Box<[T;n]>, stringifier: Stringifier) -> Self
        { Self{values: values, stringifier: Box::new(stringifier)} }
}
impl <'a,const n: usize, T: Clone> Display for StringifiableAssignment<'a,n,T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", (self.stringifier)(*self.values.clone()))
    }
}

impl <'a,const n: usize, Rule: InferenceRule> ProofStepSpecificationErrorInner for StringifiableAssignment<'a,n,OwnedSubproofInProof<Rule>>{}
impl <'a,const n: usize> ProofStepSpecificationErrorInner for StringifiableAssignment<'a,n,OwnedPropositionInProof>{}
impl <'a,const n: usize> ProofStepSpecificationErrorInner for StringifiableAssignment<'a,n,OwnedSubexpressionInProof>{}

trait ProofStepSpecificationErrorInner: Display + DynClone {}
dyn_clone::clone_trait_object!(ProofStepSpecificationErrorInner);

#[derive(Clone)]
pub struct ProofStepSpecificationError<'a>(Box<dyn ProofStepSpecificationErrorInner + 'a>);
impl <'a> ProofStepSpecificationError<'a> {
    pub fn from_inner<Inner: ProofStepSpecificationErrorInner + 'a>(inner: Inner) -> Self
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

#[cfg(test)]
mod tests {
    use crate::errors::specification_error::StringifiablePredicate;

    #[test]
    fn test_evaluate_stringifiable_predicate_on_true() {
        let predicate = |i: [i16; 2]| { i[0] < i[1] };
        let stringifier = |i: [i16; 2]| { format!("First item ({}) was not less than second item ({})",i[0],i[1]) };
        let str_pred = StringifiablePredicate::new(predicate, stringifier);
        let result = str_pred.evaluate([1,2]);
        assert!(result.is_ok())
    }

    #[test]
    fn test_evaluate_stringifiable_predicate_on_false() {
        let predicate = |i: [i16; 2]| { i[0] < i[1] };
        let stringifier = |i: [i16; 2]| { format!("First item ({}) was not less than second item ({})",i[0],i[1]) };
        let str_pred = StringifiablePredicate::new(predicate, stringifier);
        let assignment = str_pred.evaluate([2,1]).unwrap_err();
        assert_eq!(assignment.to_string(),"First item (2) was not less than second item (1)")
    }
}
