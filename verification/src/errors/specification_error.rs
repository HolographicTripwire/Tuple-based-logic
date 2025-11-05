use std::{fmt::Display};
use dyn_clone::DynClone;

use tbl_structures::{inference::InferenceRule, proof::InferenceInProof};
use tbl_textualization::structures::expressions::ExpressionStyle;

use crate::{errors::validation_error::ProofValidationError, inference_rules::InferenceVerifier};

/// Represents a predicate which maps an n-size arrays of T onto [bool] objects
pub trait Assessor<'a,I,E>: DynClone + Fn(I) -> Result<(),E> {}
impl <'a,P: 'static + Clone +  Fn(I) -> Result<(),E>, I,E> Assessor<'a,I,E> for P {}
impl <'a,I,E> Clone for Box<dyn Assessor<'a,I,E>>
    { fn clone(&self) -> Self { dyn_clone::clone_box(&**self) } }

/// Represents a stringifier that maps n-size arrays of T onto [String] objects
pub trait AssessedStringifier<'a,I: 'a + Clone,E>: DynClone + Fn(I,E) -> String {
    fn assign(self, value: I, err: E) -> StringifiableAssignment<'a, I, E> where Self: Sized + 'a
        { StringifiableAssignment::new(value, err, self) } 
}

impl <'a,S: 'a + Clone +  Fn(I,E) -> String, I: 'a + Clone,E> AssessedStringifier<'a,I,E> for S {}
impl <'a,I,E> Clone for Box<dyn AssessedStringifier<'a,I,E> + 'a>
    { fn clone(&self) -> Self { dyn_clone::clone_box(&**self) }}

/// Represents a checker that can evaluate an n-size array of T returning Ok if it matches the internal predicate, or creates a [StringifiableAssignment] from that array and the internal Stringifier otherwise
pub struct StringifiablePredicate<'a,I,E> {
    predicate: Box<dyn Assessor<'a,I,E> + 'a>,
    stringifier: Box<dyn AssessedStringifier<'a,I,E> + 'a>
}
impl <'a,I:'a + Clone,E> StringifiablePredicate<'a,I,E> {
    pub fn new<Predicate: 'a + Assessor<'a,I,E>, Stringifier: 'a + AssessedStringifier<'a,I,E>>(predicate: Predicate, stringifier: Stringifier) -> Self 
        { Self{predicate: Box::new(predicate), stringifier: Box::new(stringifier)} }
    pub fn evaluate(&self, value: I) -> Result<(),StringifiableAssignment<'a,I,E>> {
        match (self.predicate)(value.clone()) {
            Ok(_) => { Ok(()) },
            Err(err) => { Err(StringifiableAssignment::new(value, err, self.stringifier.clone())) }
        }
    }
}

#[derive(Clone)]
pub struct StringifiableAssignment<'a,I,E> {
    value: I,
    err: E,
    stringifier: Box<dyn AssessedStringifier<'a,I,E> + 'a>
}
impl <'a,I: Clone,E> StringifiableAssignment<'a,I,E> {
    fn new<Stringifier: 'a + AssessedStringifier<'a,I,E>>(value: I, err: E, stringifier: Stringifier) -> Self
        { Self{value, err, stringifier: Box::new(stringifier)} }
}
impl <'a,I: Clone,E> Display for StringifiableAssignment<'a,I,E> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", (self.stringifier)(self.value.clone()))
    }
}

pub trait ProofStepSpecificationErrorInner: Display + DynClone {}
dyn_clone::clone_trait_object!(ProofStepSpecificationErrorInner);
impl <'a,I: Clone,E:Clone> ProofStepSpecificationErrorInner for StringifiableAssignment<'a,I,E> {}

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
