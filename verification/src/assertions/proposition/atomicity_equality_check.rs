
use tbl_structures::proof::{OwnedPropositionInInference, PropositionInInference};

use crate::assertions::utils::stringify_atomicity;

pub struct PropositionAtomicityEqualityError {
    pub propositions: Vec<OwnedPropositionInInference>
}
pub fn format_proposition_atomicity_equality_error(err: PropositionAtomicityEqualityError) -> String {
    format!("Proposition atomicities expected to all be equal, but weren't; {atomicities}",
        atomicities = itertools::join(err.propositions.iter().map(|o|
            o.0.path().to_string()
            + " -> " +
            stringify_atomicity(o.0.obj().as_atom().is_ok())
        ),", ")
    )
}

/// Check that the provided [Propositions](PropositionInInference) have equal atomicity, returning an error otherwise
pub fn assert_proposition_atomicity_equality<'a>(props: &[PropositionInInference]) -> Result<(), PropositionAtomicityEqualityError> {
    let mut iter = props.iter().map(|o| o.0.obj().as_atom().is_ok());
    let first_atomicity = iter.next().expect("Cannot check atomicity equality for zero propositions");
    for nth_atomicity in iter {
        if nth_atomicity != first_atomicity { return Err(PropositionAtomicityEqualityError{
            propositions: props.into_iter().map(|x| x.clone().into_owned()).collect()
        }) }
    }
    Ok(())
}
