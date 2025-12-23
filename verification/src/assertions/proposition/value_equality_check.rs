use tbl_structures::{expressions::Proposition, proof::{OwnedPropositionInInference, PropositionInInference}};
use tbl_textualization::{helpers::styles::Style, structures::expressions::ExpressionStyle};

pub struct PropositionValueEqualityError {
    pub propositions: Vec<OwnedPropositionInInference>
}

pub fn format_proposition_value_equality_error(err: PropositionValueEqualityError, style: ExpressionStyle) -> String {
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
