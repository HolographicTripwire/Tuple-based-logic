use shared::{proposition::Proposition, term::Term};

use crate::validation_error::ProofValidationError;


pub fn prop_as_tuple(proposition: &Proposition) -> Result<&Vec<Term>,ProofValidationError> {
    proposition.0.as_tuple().or(Err(ProofValidationError::InvalidStepSpecification))
}

pub fn term_as_tuple(term: &Term) -> Result<&Vec<Term>,ProofValidationError> {
    term.as_tuple().or(Err(ProofValidationError::InvalidStepSpecification))
}

pub fn prop_as_slice(proposition: &Proposition) -> Result<&[Term],ProofValidationError> {
    proposition.0.as_slice().or(Err(ProofValidationError::InvalidStepSpecification))
}

pub fn term_as_slice(term: &Term) -> Result<&[Term],ProofValidationError> {
    term.as_slice().or(Err(ProofValidationError::InvalidStepSpecification))
}
