use shared::{atom::BuiltInAtom, proposition::Proposition};

use crate::{production_rules::tuple_or_error, ProofVerificationError};

/// Verify that the assumptions and the conclusion form a valid instance of atomicity assertion ("Atomic(Verbatim(a))" for any atom a)
pub fn verify_atomicity_assertion(assumptions: &Vec<Proposition>, conclusion: &Proposition) -> Result<(),ProofVerificationError> {
    if assumptions.len() != 0 { return Err(ProofVerificationError::InvalidStepSpecification) }
    
    // Throw an error if there are not three terms in the conclusion
    let [atomicity_head, verbatim_term] = tuple_or_error::prop_as_slice(conclusion)? else { return Err(ProofVerificationError::InvalidStepSpecification) };
    
    // Throw an error if the head of the conclusion is incorrect
    if atomicity_head != &BuiltInAtom::Atomic.into() { return Err(ProofVerificationError::InvalidStepSpecification) }

    // Throw an error if the verbatim term is not composed of two terms
    let [verbatim_head, verbatim_atom] = tuple_or_error::term_as_slice(verbatim_term)? else { return Err(ProofVerificationError::InvalidStepSpecification) };

    // Throw an error if the head of the verbatim term is incorrect
    if verbatim_head != &BuiltInAtom::Verbatim.into() { return Err(ProofVerificationError::InvalidStepSpecification) }

    // Throw an error if the verbatim atom is not actually an atom
    if verbatim_atom.as_atom().is_err() { return Err(ProofVerificationError::InvalidStepSpecification) };

    Ok(())
}
