use tbl_structures::{expressions::{Proposition, atomic::AtomicExpression}, proof::{OwnedPropositionInProofStep, PropositionInProofStep}};

pub struct PropositionValueEqualityError {
    pub propositions: Vec<OwnedPropositionInProofStep>
}
/// Check that the provided [Propositions](OwnedPropositionInProof) have equal value, returning an error otherwise
pub fn assert_proposition_value_equality<'a>(props: &[&'a PropositionInProofStep<'a>]) -> Result<Proposition, PropositionValueEqualityError> {
    let mut iter = props.iter().map(|o| o.obj );
    let first_value = iter.next().expect("Cannot check value equality for zero propositions");
    for nth_value in iter {
        if nth_value != first_value { return Err(PropositionValueEqualityError{
            propositions: props.into_iter().map(|x| (*x).clone().into()).collect()
        }) }
    }
    Ok(first_value.clone())
}





pub struct FixedLengthPropositionValueEqualityError<const N: usize> {
    pub propositions: [OwnedPropositionInProofStep; N]
}

/// Check that the provided [Propositions](PropositionInProofStep) have equal length, returning an error otherwise
pub fn assert_fixed_length_proposition_value_equality<'a,const N: usize>(exprs: &[&'a PropositionInProofStep<'a>; N]) -> Result<Proposition, FixedLengthPropositionValueEqualityError<N>> {
    if N == 0 { panic!("Cannot check value equality for zero propositions") } 
    let mut output = [&Proposition::Atomic(AtomicExpression(0)); N];  // Initialize the output array
    for i in 0..N {
        output[i] = exprs[i].obj;
        // Throw error if atomicities are not equal
        if output[i] != output[0] {
            return Err(FixedLengthPropositionValueEqualityError{
                propositions: exprs.clone().map(|x| (*x).clone().into())
            })
        }
    }
    Ok(output[0].clone())
}
