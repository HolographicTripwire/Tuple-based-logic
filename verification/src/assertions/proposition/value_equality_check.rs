use tbl_structures::{atoms::AtomId, expressions::Proposition, proof::{OwnedPropositionInInference, PropositionInInference}};
use tbl_textualization::{helpers::styles::Style, structures::expressions::PropositionStyle};

pub struct PropositionValueEqualityError {
    pub propositions: Vec<OwnedPropositionInInference>
}

pub fn format_proposition_value_equality_error(err: PropositionValueEqualityError, style: PropositionStyle) -> String {
    format!("Proposition values expected to all be equal, but weren't; {atomicities}",
        atomicities = err.propositions.iter().map(|o|
            o.0.path().to_string()
            + " -> " +
            &style.stringify(o.0.obj())
        ).collect::<Vec<_>>().join(", ")
    )
}

/// Check that the provided [Propositions](OwnedPropositionInProof) have equal value, returning an error otherwise
pub fn assert_proposition_value_equality<'a>(props: &[PropositionInInference]) -> Result<Proposition, PropositionValueEqualityError> {
    let mut iter = props.iter().map(|o| o.0.obj() );
    let first_value = iter.next().expect("Cannot check value equality for zero propositions");
    for nth_value in iter {
        if nth_value != first_value { return Err(PropositionValueEqualityError{
            propositions: props.into_iter().map(|x| x.clone().into_owned()).collect()
        }) }
    }
    Ok(first_value.clone())
}





pub struct FixedLengthPropositionValueEqualityError<const N: usize> {
    pub propositions: [OwnedPropositionInInference; N]
}
pub fn format_fixed_length_proposition_value_equality_error<const N: usize>(err: FixedLengthPropositionValueEqualityError<N>, style: PropositionStyle) -> String {
    format!("Proposition values expected to all be equal, but weren't; {atomicities}",
        atomicities = err.propositions.iter().map(|o|
            o.0.path().to_string()
            + " -> " +
            &style.stringify(o.0.obj())
        ).collect::<Vec<_>>().join(", ")
    )
}
/// Check that the provided [Propositions](PropositionInInference) have equal length, returning an error otherwise
pub fn assert_fixed_length_proposition_value_equality<'a,const N: usize>(exprs: &[PropositionInInference; N]) -> Result<Proposition, FixedLengthPropositionValueEqualityError<N>> {
    if N == 0 { panic!("Cannot check value equality for zero propositions") } 
    let mut output = [&Proposition::Atomic(AtomId(0)); N];  // Initialize the output array
    for i in 0..N {
        output[i] = exprs[i].0.obj();
        // Throw error if atomicities are not equal
        if output[i] != output[0] {
            return Err(FixedLengthPropositionValueEqualityError{
                propositions: exprs.clone().map(|x| x.clone().into_owned())
            })
        }
    }
    Ok(output[0].clone())
}
