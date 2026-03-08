
use tbl_structures::proof::{OwnedPropositionInInference, PropositionInInference};

use crate::validity::utils::stringify_atomicity;

pub struct PropositionAtomicityEqualityError {
    pub propositions: Vec<OwnedPropositionInInference>
}
pub fn format_proposition_atomicity_equality_error(err: PropositionAtomicityEqualityError) -> String {
    format!("Proposition atomicities expected to all be equal, but weren't; {atomicities}",
        atomicities = itertools::join(err.propositions.iter().map(|o|
            o.path().to_string()
            + " -> " +
            stringify_atomicity(o.obj().as_atom().is_ok())
        ),", ")
    )
}

/// Check that the provided [Propositions](PropositionInInference) have equal atomicity, returning an error otherwise
pub fn assert_proposition_atomicity_equality<'a>(props: &[&'a PropositionInInference<'a>]) -> Result<(), PropositionAtomicityEqualityError> {
    let mut iter = props.iter().map(|o| o.obj().as_atom().is_ok());
    let first_atomicity = iter.next().expect("Cannot check atomicity equality for zero propositions");
    for nth_atomicity in iter {
        if nth_atomicity != first_atomicity { return Err(PropositionAtomicityEqualityError{
            propositions: props.into_iter().map(|x| (*x).clone().into_owned()).collect()
        }) }
    }
    Ok(())
}

pub struct FixedLengthPropositionAtomicityEqualityError<const N: usize> {
    pub propositions: [OwnedPropositionInInference; N]
}
pub fn format_fixed_length_proposition_atomicity_equality_error<const N: usize>(err: FixedLengthPropositionAtomicityEqualityError<N>) -> String {
    format!("Proposition atomicities expected to all be equal, but weren't; {atomicities}",
        atomicities = itertools::join(err.propositions.iter().map(|o|
            o.path().to_string()
            + " -> " +
            stringify_atomicity(o.obj().as_atom().is_ok())
        ),", ")
    )
}
/// Check that the provided [Propositions](PropositionInInference) have equal atomicity, returning an error otherwise
pub fn assert_fixed_length_proposition_atomicity_equality<'a,const N: usize>(exprs: &[&'a PropositionInInference<'a>; N]) -> Result<bool, FixedLengthPropositionAtomicityEqualityError<N>> {
    if N == 0 { panic!("Cannot check atomicity equality for zero propositions") } 
    let mut output = [false; N];  // Initialize the output array
    for i in 0..N {
        output[i] = exprs[i].obj().as_atom().is_ok();
        // Throw error if atomicities are not equal
        if output[i] != output[0] {
            return Err(FixedLengthPropositionAtomicityEqualityError{
                propositions: exprs.clone().map(|x| x.clone().into_owned())
            })
        }
    }
    Ok(output[0])
}
