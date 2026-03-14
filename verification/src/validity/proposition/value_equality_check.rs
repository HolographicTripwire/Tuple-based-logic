use tbl_structures::{atoms::AtomId, expressions::Proposition, proof::inference::{OwnedPropositionInInference, PropositionInInference}};

pub struct PropositionValueEqualityError {
    pub propositions: Vec<OwnedPropositionInInference>
}
/// Check that the provided [Propositions](OwnedPropositionInProof) have equal value, returning an error otherwise
pub fn assert_proposition_value_equality<'a>(props: &[&'a PropositionInInference<'a>]) -> Result<Proposition, PropositionValueEqualityError> {
    let mut iter = props.iter().map(|o| o.obj() );
    let first_value = iter.next().expect("Cannot check value equality for zero propositions");
    for nth_value in iter {
        if nth_value != first_value { return Err(PropositionValueEqualityError{
            propositions: props.into_iter().map(|x| (*x).clone().into_owned()).collect()
        }) }
    }
    Ok(first_value.clone())
}





pub struct FixedLengthPropositionValueEqualityError<const N: usize> {
    pub propositions: [OwnedPropositionInInference; N]
}

/// Check that the provided [Propositions](PropositionInInference) have equal length, returning an error otherwise
pub fn assert_fixed_length_proposition_value_equality<'a,const N: usize>(exprs: &[&'a PropositionInInference<'a>; N]) -> Result<Proposition, FixedLengthPropositionValueEqualityError<N>> {
    if N == 0 { panic!("Cannot check value equality for zero propositions") } 
    let mut output = [&Proposition::Atomic(AtomId(0)); N];  // Initialize the output array
    for i in 0..N {
        output[i] = exprs[i].obj();
        // Throw error if atomicities are not equal
        if output[i] != output[0] {
            return Err(FixedLengthPropositionValueEqualityError{
                propositions: exprs.clone().map(|x| (*x).clone().into_owned())
            })
        }
    }
    Ok(output[0].clone())
}
