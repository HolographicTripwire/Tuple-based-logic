use shared::{atom::BuiltInAtom, proposition::Proposition};

use crate::{inference_rules::tuple_or_error, ProofValidationError};

use super::resolve_verbatim;

/// Verify that the assumptions and the conclusion form a valid instance of atomicity assertion ("(Verbatim(a) != Verbatim(b))" for any two different atoms a and b)
pub fn verify_atom_differentiation(assumptions: &Vec<Proposition>, conclusions: &Vec<Proposition>) -> Result<(),ProofValidationError> {
    // Throw an error if there is not exactly one conclusion
    let [conclusion] = conclusions.as_slice() else { return Err(ProofValidationError::InvalidStepSpecification) };
    // Throw ane rror if the rule has any assumptions (this rule requires none)
    if assumptions.len() != 0 { return Err(ProofValidationError::InvalidStepSpecification) }
    
    // Throw an error if there are not three terms in the conclusion
    let [nonidentity_head, identity_left, identity_right] = tuple_or_error::prop_as_slice(conclusion)? else { return Err(ProofValidationError::InvalidStepSpecification) };
    
    // Throw an error if the head of the conclusion is incorrect
    if nonidentity_head != &BuiltInAtom::NonIdentity.into() { return Err(ProofValidationError::InvalidStepSpecification) }

    // Throw an error if either of the verbatim terms do not resolve as Verbatim
    let left_verbatim_atom = resolve_verbatim(identity_left)?;
    let right_verbatim_atom = resolve_verbatim(identity_right)?;

    // Throw an error if either of the verbatim atoms is not actually an atom
    if left_verbatim_atom.as_atom().is_err() { return Err(ProofValidationError::InvalidStepSpecification) };
    if right_verbatim_atom.as_atom().is_err() { return Err(ProofValidationError::InvalidStepSpecification) };
    
    // Throw an error if the atoms aree actually identical
    if left_verbatim_atom == right_verbatim_atom { return Err(ProofValidationError::InvalidStepSpecification) };

    Ok(())
}
