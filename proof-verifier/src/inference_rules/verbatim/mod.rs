mod atomicity_assertion;
mod atom_differentiation;
mod tuple_appendation;

pub use atomicity_assertion::verify_atomicity_assertion;
pub use atom_differentiation::verify_atom_differentiation;
use shared::{atoms::BuiltInAtom, propositions::Term};
pub use tuple_appendation::verify_tuple_appendation;

use crate::validation_error::ProofValidationError;

use super::tuple_or_error;

fn resolve_verbatim(verbatim_term: &Term) -> Result<&Term,ProofValidationError>{
    let [verbatim_head, verbatim_tail] = tuple_or_error::term_as_slice(verbatim_term)? else { return Err(ProofValidationError::InvalidStepSpecification) };
    if verbatim_head != &BuiltInAtom::Verbatim.into() { return Err(ProofValidationError::InvalidStepSpecification) }
    Ok(verbatim_tail)
}
