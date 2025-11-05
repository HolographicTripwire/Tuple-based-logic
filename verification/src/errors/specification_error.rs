use std::{fmt::Display};
use dyn_clone::DynClone;

use tbl_structures::{inference::InferenceRule, proof::InferenceInProof};
use tbl_textualization::structures::expressions::ExpressionStyle;

use crate::{errors::validation_error::ProofValidationError, inference_rules::InferenceVerifier};

/// Represents a predicate which maps an n-size arrays of T onto [bool] objects
pub trait NaryPredicate<'a,I>: DynClone + Fn(I) -> bool {}
impl <'a,P: 'static + Clone +  Fn(I) -> bool, I> NaryPredicate<'a,I> for P {}
impl <'a,I> Clone for Box<dyn NaryPredicate<'a,I>>
    { fn clone(&self) -> Self { dyn_clone::clone_box(&**self) } }

/// Represents a stringifier that maps n-size arrays of T onto [String] objects
pub trait NaryStringifier<'a,I: 'a + Clone>: DynClone + Fn(I) -> String {
    fn assign(self, value: I) -> StringifiableAssignment<'a, I> where Self: Sized + 'a
        { StringifiableAssignment::new(value, self) } 
}

impl <'a,S: 'a + Clone +  Fn(I) -> String, I: 'a + Clone> NaryStringifier<'a,I> for S {}
impl <'a,I> Clone for Box<dyn NaryStringifier<'a,I> + 'a>
    { fn clone(&self) -> Self { dyn_clone::clone_box(&**self) }}

/// Represents a checker that can evaluate an n-size array of T returning Ok if it matches the internal predicate, or creates a [StringifiableAssignment] from that array and the internal Stringifier otherwise
pub struct StringifiablePredicate<'a,I> {
    predicate: Box<dyn NaryPredicate<'a,I> + 'a>,
    stringifier: Box<dyn NaryStringifier<'a,I> + 'a>
}
impl <'a,I:'a + Clone> StringifiablePredicate<'a,I> {
    pub fn new<Predicate: 'a + NaryPredicate<'a,I>, Stringifier: 'a + NaryStringifier<'a,I>>(predicate: Predicate, stringifier: Stringifier) -> Self 
        { Self{predicate: Box::new(predicate), stringifier: Box::new(stringifier)} }
    pub fn evaluate(&self, value: I) -> Result<(),StringifiableAssignment<'a,I>> {
        if (self.predicate)(value.clone()) { Ok(()) }
        else { Err(StringifiableAssignment::new(value, self.stringifier.clone())) }
    }
}

#[derive(Clone)]
pub struct StringifiableAssignment<'a,I> {
    value: I,
    stringifier: Box<dyn NaryStringifier<'a,I> + 'a>
}
impl <'a,I: Clone> StringifiableAssignment<'a,I> {
    fn new<Stringifier: 'a + NaryStringifier<'a,I>>(value: I, stringifier: Stringifier) -> Self
        { Self{value, stringifier: Box::new(stringifier)} }
}
impl <'a,I: Clone> Display for StringifiableAssignment<'a,I> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", (self.stringifier)(self.value.clone()))
    }
}

pub trait ProofStepSpecificationErrorInner: Display + DynClone {}
dyn_clone::clone_trait_object!(ProofStepSpecificationErrorInner);
impl <'a,I: Clone> ProofStepSpecificationErrorInner for StringifiableAssignment<'a,I> {}

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
