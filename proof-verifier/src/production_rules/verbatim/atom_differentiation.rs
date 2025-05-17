use shared::{atom::BuiltInAtom, proposition::Proposition};

use crate::{production_rules::tuple_or_error, ProofValidationError};

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

    // Throw an error if either of the verbatim terms are not composed of two terms
    let [left_verbatim_head, left_verbatim_atom] = tuple_or_error::term_as_slice(identity_left)? else { return Err(ProofValidationError::InvalidStepSpecification) };
    let [right_verbatim_head, right_verbatim_atom] = tuple_or_error::term_as_slice(identity_right)? else { return Err(ProofValidationError::InvalidStepSpecification) };

    // Throw an error if either of the heads of the verbatim term is incorrect
    if left_verbatim_head != &BuiltInAtom::Verbatim.into() { return Err(ProofValidationError::InvalidStepSpecification) }
    if right_verbatim_head != &BuiltInAtom::Verbatim.into() { return Err(ProofValidationError::InvalidStepSpecification) }

    // Throw an error if either of the verbatim atoms is not actually an atom
    if left_verbatim_atom.as_atom().is_err() { return Err(ProofValidationError::InvalidStepSpecification) };
    if right_verbatim_atom.as_atom().is_err() { return Err(ProofValidationError::InvalidStepSpecification) };
    
    // Throw an error if the atoms aree actually identical
    if left_verbatim_atom == right_verbatim_atom { return Err(ProofValidationError::InvalidStepSpecification) };

    Ok(())
}
