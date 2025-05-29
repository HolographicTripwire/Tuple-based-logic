use tbl_structures::{atoms::BuiltInAtom, propositions::Proposition};

use crate::{inference_rules::{TUPLE_OR_ERROR}, ProofValidationError};

use super::resolve_verbatim;

/// Verify that the assumptions and the conclusion form a valid instance of atomicity assertion ("Atomic(Verbatim(a))" for any atom a)
pub fn verify_atomicity_assertion(assumptions: &Vec<Proposition>, conclusions: &Vec<Proposition>) -> Result<(),ProofValidationError> {
    // Throw an error if there is not exactly one conclusion
    let [conclusion] = conclusions.as_slice() else { return Err(ProofValidationError::InvalidStepSpecification) };
    // Throw ane rror if the rule has any assumptions (this rule requires none)
    if assumptions.len() != 0 { return Err(ProofValidationError::InvalidStepSpecification) }
    
    // Throw an error if there are not three expressions in the conclusion
    let [atomicity_head, verbatim_expr] = TUPLE_OR_ERROR.prop_as_slice(conclusion)? else { return Err(ProofValidationError::InvalidStepSpecification) };
    
    // Throw an error if the head of the conclusion is incorrect
    if atomicity_head != &BuiltInAtom::Atomic.into() { return Err(ProofValidationError::InvalidStepSpecification) }

    // Throw an error if the verbatim expression does not resolve to as Verbatim
    let verbatim_atom = resolve_verbatim(verbatim_expr)?;
    
    // Throw an error if the verbatim atom is not actually an atom
    if verbatim_atom.as_atom().is_err() { return Err(ProofValidationError::InvalidStepSpecification) };

    Ok(())
}
