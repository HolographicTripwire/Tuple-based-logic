use tbl_structures::{atoms::BuiltInAtom, propositions::{Proposition, Expression}};

use crate::{inference_rules::TUPLE_OR_ERROR, ProofValidationError};

use super::resolve_verbatim;

/// Verify that the assumptions and the conclusion form a valid instance of atomicity assertion ("Append(Verbatim((v1,v2,v3,...,vn)),Verbatim(vm)) = Verbatim((v1,v2,v3,...,vn,vm))" for any (v1,v2,v3,...,vn) and vm)
pub fn verify_tuple_appendation(assumptions: &Vec<Proposition>, conclusions: &Vec<Proposition>) -> Result<(),ProofValidationError> {
    // Throw an error if there is not exactly one conclusion
    let [conclusion] = conclusions.as_slice() else { return Err(ProofValidationError::InvalidStepSpecification) };
    // Throw ane rror if the rule has any assumptions (this rule requires none)
    if assumptions.len() != 0 { return Err(ProofValidationError::InvalidStepSpecification) }
    
    // Throw an error if there are not three exprs in the conclusion
    let [identity_head, appendation_expr, appended] = TUPLE_OR_ERROR.as_slice(conclusion)? else { return Err(ProofValidationError::InvalidStepSpecification) };
    let [appendation_head, append_to, to_append] = TUPLE_OR_ERROR.as_slice(appendation_expr)? else { return Err(ProofValidationError::InvalidStepSpecification) };    

    // Throw an error if the head of the conclusion is incorrect
    if identity_head != &BuiltInAtom::Identity.into() { return Err(ProofValidationError::InvalidStepSpecification) }
    if appendation_head != &BuiltInAtom::Concatenate.into() { return Err(ProofValidationError::InvalidStepSpecification) }
    
    let append_to_verbatim = TUPLE_OR_ERROR.as_tuple(resolve_verbatim(append_to)?)?;
    let to_append_verbatim = resolve_verbatim(to_append)?;
    let appended_verbatim = resolve_verbatim(appended)?;

    // Throw an error if the exprs aren't actually identical
    if &resolve_appendation(append_to_verbatim.clone(), to_append_verbatim) != appended_verbatim { return Err(ProofValidationError::InvalidStepSpecification) };

    Ok(())
}

fn resolve_appendation(mut append_to: Vec<Expression>, to_append: &Expression) -> Expression {
    append_to.push(to_append.clone()); Expression::Tuple(append_to)
}
