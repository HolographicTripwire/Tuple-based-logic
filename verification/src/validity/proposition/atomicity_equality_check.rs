use tbl_structures::proof::inference::{OwnedPropositionInInference, PropositionInInference};

pub struct PropositionAtomicityEqualityError {
    pub propositions: Vec<OwnedPropositionInInference>
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
