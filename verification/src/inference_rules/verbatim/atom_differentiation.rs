use tbl_structures::{atoms::BuiltInAtom, propositions::Proposition};

use crate::{inference_rules::TUPLE_OR_ERROR, ProofValidationError};

use super::resolve_verbatim;

/// Verify that the assumptions and the conclusion form a valid instance of atomicity assertion ("Not (Verbatim(a) = Verbatim(b))" for any two different atoms a and b)
pub fn verify_atom_differentiation(assumptions: &Vec<Proposition>, conclusions: &Vec<Proposition>) -> Result<(),ProofValidationError> {
    // Throw an error if there is not exactly one conclusion
    let [conclusion] = conclusions.as_slice() else { return Err(ProofValidationError::InvalidStepSpecification) };
    // Throw ane rror if the rule has any assumptions (this rule requires none)
    if assumptions.len() != 0 { return Err(ProofValidationError::InvalidStepSpecification) }
    
    // Throw an error if there are not two expressions in the conclusion
    let [negation_head, identity] = TUPLE_OR_ERROR.prop_as_slice(conclusion)? else { return Err(ProofValidationError::InvalidStepSpecification) };
    // Throw an error if the head of the conclusion is incorrect
    if negation_head != &BuiltInAtom::Negation.into() { return Err(ProofValidationError::InvalidStepSpecification) }
    
    // Throw an error if there are not three expressions in the identity
    let [identity_head, identity_left, identity_right] = TUPLE_OR_ERROR.expr_as_slice(identity)? else { return Err(ProofValidationError::InvalidStepSpecification) };
    // Throw an error if the head of the identity is incorrect
    if identity_head != &BuiltInAtom::Identity.into() { return Err(ProofValidationError::InvalidStepSpecification) }

    // Throw an error if either of the verbatim expressions do not resolve as Verbatim
    let left_verbatim_atom = resolve_verbatim(identity_left)?;
    let right_verbatim_atom = resolve_verbatim(identity_right)?;

    // Throw an error if either of the verbatim atoms is not actually an atom
    if left_verbatim_atom.as_atom().is_err() { return Err(ProofValidationError::InvalidStepSpecification) };
    if right_verbatim_atom.as_atom().is_err() { return Err(ProofValidationError::InvalidStepSpecification) };
    
    // Throw an error if the atoms aree actually identical
    if left_verbatim_atom == right_verbatim_atom { return Err(ProofValidationError::InvalidStepSpecification) };

    Ok(())
}
