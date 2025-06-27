use tbl_structures::{inference::{path::SubexpressionInInference, Inference, InferenceRule}, propositions::Expression};

use crate::inference_rules::error::ProofStepSpecificationError;

pub fn assumptions_as_sized_slice<'a, const expected_size: usize, Rule: InferenceRule>(inference: &'a Inference<Rule>) -> Result<Box<[SubexpressionInInference<'a>; expected_size]>,ProofStepSpecificationError> {
    expressions_as_sized_slice::<expected_size>(
        inference.get_located_assumptions(), 
        ProofStepSpecificationError::WrongAssumptionCount(expected_size)
    )
}

pub fn conclusions_as_sized_slice<'a, const expected_size: usize, Rule: InferenceRule>(inference: &'a Inference<Rule>) -> Result<Box<[SubexpressionInInference<'a>; expected_size]>,ProofStepSpecificationError> {
    expressions_as_sized_slice::<expected_size>(
        inference.get_located_conclusions(),
        ProofStepSpecificationError::WrongConclusionCount(expected_size)
    )
}

pub fn expression_as_slice<'a>(expression: &'a SubexpressionInInference) -> Result<Box<[SubexpressionInInference<'a>]>,ProofStepSpecificationError> {
    if let Ok(subexpressions) = expression.subexpressions() { Ok(Box::from(subexpressions)) }
    else { Err(ProofStepSpecificationError::WrongAtomicity(expression.path().clone(), false)) }
}

pub fn expression_as_sized_slice<'a, const expected_size: usize>(expression: &'a SubexpressionInInference) -> Result<Box<[SubexpressionInInference<'a>; expected_size]>,ProofStepSpecificationError> {
    expressions_as_sized_slice::<expected_size>(expression_as_slice(expression)?.to_vec(),ProofStepSpecificationError::WrongLength(expression.path().clone(), expected_size))
}
pub fn expressions_as_sized_slice<'a, const expected_size: usize>(expressions: Vec<SubexpressionInInference<'a>>, err: ProofStepSpecificationError) -> Result<Box<[SubexpressionInInference<'a>; expected_size]>,ProofStepSpecificationError> {
    let result: Result<&[SubexpressionInInference<'a>; expected_size],_> = expressions.as_slice().try_into();
    match result {
        Ok(sliced) => Ok(Box::new(sliced.clone())),
        Err(_) => Err(err),
    }
}

pub fn assert_expression_value<'a,'b>(expression: &SubexpressionInInference<'a>, check: impl Into<&'b Expression>) -> Result<(),ProofStepSpecificationError> {
    let check = check.into();
    if expression.expression() == check { Ok(()) }
    else { Err(ProofStepSpecificationError::WrongValue(expression.path().clone(), check.clone())) }
}

pub fn assert_expression_atomicity<'a,'b>(expression: &SubexpressionInInference<'a>, should_be_atomic: bool) -> Result<(),ProofStepSpecificationError> {
    if expression.expression().as_atom().is_ok() == should_be_atomic { Ok(()) }
    else { Err(ProofStepSpecificationError::WrongAtomicity(expression.path().clone(), should_be_atomic)) }
}


pub fn assert_length_match(a: &SubexpressionInInference, b: &SubexpressionInInference) -> Result<(),ProofStepSpecificationError> {
    let error = Err(ProofStepSpecificationError::MismatchedLengths(a.path().clone(), b.path().clone()));
    match (a.subexpressions(), b.subexpressions()) {
        (Ok(a), Ok(b)) => if a.len() == b.len() { Ok(()) } else { error },
        (Err(_), Err(_)) => Ok(()),
        _ => error,
    }
}

pub fn assert_value_match(a: &SubexpressionInInference, b: &SubexpressionInInference) -> Result<(),ProofStepSpecificationError> {
    if a.expression() == b.expression() { Ok(()) }
    else { Err(ProofStepSpecificationError::MismatchedValues(a.path().clone(), b.path().clone())) }
}
