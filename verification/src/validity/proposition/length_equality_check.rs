use tbl_structures::sequential_proofs::{OwnedPropositionInProofStep, PropositionInProofStep};

pub struct PropositionLengthEqualityError {
    pub propositions: Vec<OwnedPropositionInProofStep>
}

/// Check that the provided [Propositions](PropositionInProofStep) have equal length, returning an error otherwise
pub fn assert_proposition_length_equality<'a>(props: &[&'a PropositionInProofStep<'a>]) -> Result<Option<usize>, PropositionLengthEqualityError> {
    let mut iter = props.iter().map(|o| match o.obj.as_slice() {
        Ok(propositions) => Some(propositions.len()),
        Err(_) => None,
    });
    let first_length = iter.next().expect("Cannot check length equality for zero propositions");
    for nth_length in iter {
        if nth_length != first_length { return Err(PropositionLengthEqualityError {
            propositions: props.into_iter().map(|x| (*x).clone().into()).collect()
        }) }
    }
    Ok(first_length)
}

pub struct FixedLengthPropositionLengthEqualityError<const N: usize> {
    pub propositions: [OwnedPropositionInProofStep; N]
}
/// Check that the provided [Propositions](PropositionInProofStep) have equal length, returning an error otherwise
pub fn assert_fixed_length_proposition_length_equality<'a,const N: usize>(exprs: &[&'a PropositionInProofStep<'a>; N]) -> Result<Option<usize>, FixedLengthPropositionLengthEqualityError<N>> {
    if N == 0 { panic!("Cannot check length equality for zero propositions") } 
    let mut output = [None; N];  // Initialize the output array
    for i in 0..N {
        output[i] = exprs[i].obj.len();
        // Throw error if atomicities are not equal
        if output[i] != output[0] {
            return Err(FixedLengthPropositionLengthEqualityError{
                propositions: exprs.clone().map(|x| (*x).clone().into())
            })
        }
    }
    Ok(output[0])
}
