use std::collections::HashSet;

use tbl_structures::proof::{OwnedPropositionInInference, PropositionInInference};
use tbl_textualization::{helpers::styles::Style, structures::expressions::ExpressionStyle};

pub struct PropositionValueInequalityError {
    pub propositions: Vec<OwnedPropositionInInference>,
}


pub fn format_proposition_value_inequality_error(err: PropositionValueInequalityError, style: ExpressionStyle) -> String {
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
    let iter = props.iter().map(|o| match o.0.obj().as_slice() {
        Ok(propositions) => Some(propositions.len()),
        Err(_) => None,
    });
    let mut values = HashSet::new();
    for value in iter
        { if !values.insert(value) { return Err(PropositionValueInequalityError{
            propositions: props.into_iter().map(|x| x.clone().into_owned()).collect()
        }); } }
    Ok(())
}
