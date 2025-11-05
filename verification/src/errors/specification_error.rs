use std::{fmt::Display};
use dyn_clone::DynClone;

use tbl_structures::{inference::InferenceRule, path_composites::{OwnedExpressionInProof, OwnedPropositionInProof}, proof::{InferenceInProof, OwnedInferenceInProof, OwnedProofInProof}};
use tbl_textualization::structures::expressions::ExpressionStyle;

use crate::{errors::validation_error::ProofValidationError, inference_rules::InferenceVerifier};

/// Represents a predicate which maps an n-size arrays of T onto [bool] objects
pub trait NaryPredicate<'a,const N: usize,T>: DynClone + Fn([T; N]) -> bool {}
impl <'a,P: 'static + Clone +  Fn([T; N]) -> bool, const N: usize,T> NaryPredicate<'a,N,T> for P {}
impl <'a,const N: usize, T> Clone for Box<dyn NaryPredicate<'a,N,T>>
    { fn clone(&self) -> Self { dyn_clone::clone_box(&**self) } }

/// Represents a stringifier that maps n-size arrays of T onto [String] objects
pub trait NaryStringifier<'a,const N: usize,T: 'a + Clone>: DynClone + Fn([T; N]) -> String {
    fn assign(self, values: [T; N]) -> StringifiableAssignment<'a, N, T> where Self: Sized + 'a
        { StringifiableAssignment::new(Box::new(values), self) } 
}

impl <'a,S: 'a + Clone +  Fn([T; N]) -> String, const N: usize,T: 'a + Clone> NaryStringifier<'a,N,T> for S {}
impl <'a,const N: usize, T> Clone for Box<dyn NaryStringifier<'a,N,T> + 'a>
    { fn clone(&self) -> Self { dyn_clone::clone_box(&**self) }}

/// Represents a checker that can evaluate an n-size array of T returning Ok if it matches the internal predicate, or creates a [StringifiableAssignment] from that array and the internal Stringifier otherwise
pub struct StringifiablePredicate<'a,const N: usize, T> {
    predicate: Box<dyn NaryPredicate<'a,N,T> + 'a>,
    stringifier: Box<dyn NaryStringifier<'a,N,T> + 'a>
}
impl <'a,const N: usize, T:'a + Clone> StringifiablePredicate<'a,N,T> {
    pub fn new<Predicate: 'a + NaryPredicate<'a,N,T>, Stringifier: 'a + NaryStringifier<'a,N,T>>(predicate: Predicate, stringifier: Stringifier) -> Self 
        { Self{predicate: Box::new(predicate), stringifier: Box::new(stringifier)} }
    pub fn evaluate(&self, values: [T;N]) -> Result<(),StringifiableAssignment<'a,N,T>> {
        if (self.predicate)(values.clone()) { Ok(()) }
        else { Err(StringifiableAssignment::new(Box::new(values), self.stringifier.clone())) }
    }
}

#[derive(Clone)]
pub struct StringifiableAssignment<'a,const N: usize, T> {
    values: Box<[T;N]>,
    stringifier: Box<dyn NaryStringifier<'a,N,T> + 'a>
}
impl <'a,const N: usize, T: Clone> StringifiableAssignment<'a,N,T> {
    fn new<Stringifier: 'a + NaryStringifier<'a,N,T>>(values: Box<[T;N]>, stringifier: Stringifier) -> Self
        { Self{values: values, stringifier: Box::new(stringifier)} }
}
impl <'a,const N: usize, T: Clone> Display for StringifiableAssignment<'a,N,T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", (self.stringifier)(*self.values.clone()))
    }
}

impl <'a,const N: usize, Rule: InferenceRule> ProofStepSpecificationErrorInner for StringifiableAssignment<'a,N,OwnedProofInProof<Rule>>{}
impl <'a,const N: usize,Rule: InferenceRule> ProofStepSpecificationErrorInner for StringifiableAssignment<'a,N,OwnedInferenceInProof<Rule>>{}
impl <'a,const N: usize> ProofStepSpecificationErrorInner for StringifiableAssignment<'a,N,OwnedPropositionInProof>{}
impl <'a,const N: usize> ProofStepSpecificationErrorInner for StringifiableAssignment<'a,N,OwnedExpressionInProof>{}

pub trait ProofStepSpecificationErrorInner: Display + DynClone {}
dyn_clone::clone_trait_object!(ProofStepSpecificationErrorInner);

#[derive(Clone)]
pub struct ProofStepSpecificationError<'a>(Box<dyn ProofStepSpecificationErrorInner + 'a>);
impl <'a> ProofStepSpecificationError<'a> {
    pub fn from_inner<Inner: ProofStepSpecificationErrorInner + 'a>(inner: Inner) -> Self
        { Self(Box::new(inner)) }
}
impl <'a> Display for ProofStepSpecificationError<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { write!(f, "{}", self.0) }
}

pub fn verify_inference<'a, Rule: VerifiableInferenceRule>(inference: &InferenceInProof<Rule>, style: ExpressionStyle<'a>) -> Result<(),ProofValidationError<'a>> {
    let verifier = Rule::get_verifier(&inference.0.obj().inference_type);
    match verifier(inference, style) {
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
