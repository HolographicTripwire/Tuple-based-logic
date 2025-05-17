use shared::{proposition::Proposition, term::Term};

use crate::verification_error::ProofVerificationError;


pub fn prop_as_tuple(proposition: &Proposition) -> Result<&Vec<Term>,ProofVerificationError> {
    proposition.0.as_tuple().or(Err(ProofVerificationError::InvalidStepSpecification))
}

pub fn term_as_tuple(term: &Term) -> Result<&Vec<Term>,ProofVerificationError> {
    term.as_tuple().or(Err(ProofVerificationError::InvalidStepSpecification))
}

pub fn prop_as_slice(proposition: &Proposition) -> Result<&[Term],ProofVerificationError> {
    proposition.0.as_slice().or(Err(ProofVerificationError::InvalidStepSpecification))
}

pub fn term_as_slice(term: &Term) -> Result<&[Term],ProofVerificationError> {
    term.as_slice().or(Err(ProofVerificationError::InvalidStepSpecification))
}
