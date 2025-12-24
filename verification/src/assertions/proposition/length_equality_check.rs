
use tbl_structures::proof::{OwnedPropositionInInference, PropositionInInference};

use crate::assertions::utils::stringify_length;


pub struct PropositionLengthEqualityError {
    pub propositions: Vec<OwnedPropositionInInference>
}

pub fn format_proposition_length_equality_error(err: PropositionLengthEqualityError) -> String {
    format!("Proposition lengths expected to all be equal, but weren't; {atomicities}",
        atomicities = err.propositions.iter().map(|o|
            o.0.path().to_string()
            + " -> " +
            &stringify_length(o.0.obj())
        ).collect::<Vec<_>>().join(", ")
    )
}

/// Check that the provided [Propositions](PropositionInInference) have equal length, returning an error otherwise
pub fn assert_proposition_length_equality<'a>(props: &[PropositionInInference]) -> Result<Option<usize>, PropositionLengthEqualityError> {
    let mut iter = props.iter().map(|o| match o.0.obj().as_slice() {
        Ok(propositions) => Some(propositions.len()),
        Err(_) => None,
    });
    let first_length = iter.next().expect("Cannot check length equality for zero propositions");
    for nth_length in iter {
        if nth_length != first_length { return Err(PropositionLengthEqualityError {
            propositions: props.into_iter().map(|x| x.clone().into_owned()).collect()
        }) }
    }
    Ok(first_length)
}

pub struct FixedLengthPropositionLengthEqualityError<const N: usize> {
    pub propositions: [OwnedPropositionInInference; N]
}
pub fn format_fixed_length_proposition_length_equality_error<const N: usize>(err: FixedLengthPropositionLengthEqualityError<N>) -> String {
    format!("Proposition lengths expected to all be equal, but weren't; {atomicities}",
        atomicities = err.propositions.iter().map(|o|
            o.0.path().to_string()
            + " -> " +
            &stringify_length(o.0.obj())
        ).collect::<Vec<_>>().join(", ")
    )
}
/// Check that the provided [Propositions](PropositionInInference) have equal length, returning an error otherwise
pub fn assert_fixed_length_proposition_length_equality<'a,const N: usize>(exprs: &[PropositionInInference; N]) -> Result<Option<usize>, FixedLengthPropositionLengthEqualityError<N>> {
    if N == 0 { panic!("Cannot check length equality for zero propositions") } 
    let mut output = [None; N];  // Initialize the output array
    for i in 0..N {
        output[i] = exprs[i].0.obj().len();
        // Throw error if atomicities are not equal
        if output[i] != output[0] {
            return Err(FixedLengthPropositionLengthEqualityError{
                propositions: exprs.clone().map(|x| x.clone().into_owned())
            })
        }
    }
    Ok(output[0])
}
