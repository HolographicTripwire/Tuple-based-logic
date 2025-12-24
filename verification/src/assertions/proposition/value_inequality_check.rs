use std::collections::HashSet;

use tbl_structures::proof::{OwnedPropositionInInference, PropositionInInference};
use tbl_textualization::{helpers::styles::Style, structures::expressions::PropositionStyle};

pub struct PropositionValueInequalityError {
    pub propositions: Vec<OwnedPropositionInInference>,
}

pub fn format_proposition_value_inequality_error(err: PropositionValueInequalityError, style: PropositionStyle) -> String {
    format!("Proposition values expected to all be inequal, but weren't; {values}",
        values = err.propositions.iter().map(|o|
            o.0.path().to_string()
            + " -> " +
            &style.stringify(o.0.obj())
        ).collect::<Vec<_>>().join(", ")
    )
}

/// Check that the provided [Propositions](PropositionInInference) have inequal value, returning an error otherwise
pub fn assert_proposition_value_inequality<'a>(props: &[PropositionInInference]) -> Result<(), PropositionValueInequalityError> {
    let iter = props.iter().map(|o| o.0.obj());
    let mut values = HashSet::new();
    for value in iter
        { if !values.insert(value) { return Err(PropositionValueInequalityError{
            propositions: props.into_iter().map(|x| x.clone().into_owned()).collect()
        }); } }
    Ok(())
}




pub struct FixedLengthPropositionValueInequalityError<const N: usize> {
    pub propositions: [OwnedPropositionInInference; N]
}
pub fn format_fixed_length_proposition_value_inequality_error<const N: usize>(err: FixedLengthPropositionValueInequalityError<N>, style: PropositionStyle) -> String {
    format!("Proposition lengths expected to all be equal, but weren't; {atomicities}",
        atomicities = err.propositions.iter().map(|o|
            o.0.path().to_string()
            + " -> " +
            &style.stringify(o.0.obj())
        ).collect::<Vec<_>>().join(", ")
    )
}
/// Check that the provided [Propositions](PropositionInInference) have inequal length, returning an error otherwise
pub fn assert_fixed_length_proposition_value_inequality<'a,const N: usize>(exprs: &[PropositionInInference; N]) -> Result<(), FixedLengthPropositionValueInequalityError<N>> {
    if N == 0 { panic!("Cannot check length inequality for zero propositions") } 
    let iter = exprs.iter().map(|o| o.0.obj());
    let mut values = HashSet::new();
    for value in iter
        { if !values.insert(value) { return Err(FixedLengthPropositionValueInequalityError {
            propositions: exprs.clone().map(|x| x.clone().into_owned())
        }); } }
    Ok(())
}
