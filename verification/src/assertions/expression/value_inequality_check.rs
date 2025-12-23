use std::collections::HashSet;

use tbl_structures::proof::{OwnedPropositionInInference, PropositionInInference};
use tbl_textualization::{helpers::styles::Style, structures::expressions::ExpressionStyle};

pub struct PropositionValueInequalityError {
    pub expressions: Vec<OwnedPropositionInInference>,
}


pub fn format_expression_value_inequality_error(err: PropositionValueInequalityError, style: ExpressionStyle) -> String {
    format!("Proposition values expected to all be inequal, but weren't; {values}",
        values = err.expressions.iter().map(|o|
            o.0.path().to_string()
            + " -> " +
            &style.stringify(o.0.obj())
        ).collect::<Vec<_>>().join(", ")
    )
}


/// Check that the provided [Propositions](PropositionInInference) have inequal value, returning an error otherwise
pub fn assert_expression_value_inequality<'a>(exprs: &[PropositionInInference]) -> Result<(), PropositionValueInequalityError> {
    let iter = exprs.iter().map(|o| match o.0.obj().as_slice() {
        Ok(expressions) => Some(expressions.len()),
        Err(_) => None,
    });
    let mut values = HashSet::new();
    for value in iter
        { if !values.insert(value) { return Err(PropositionValueInequalityError{
            expressions: exprs.into_iter().map(|x| x.clone().into_owned()).collect()
        }); } }
    Ok(())
}
