use tbl_structures::{atoms::BuiltInAtom, expressions::Expression, proof::InferenceInProof};
use tbl_textualization::structures::expressions::ExpressionStyle;


use crate::{assertions::*, errors::specification_error::ProofStepSpecificationError, inference_rules::StandardInferenceRule};

use super::resolve_verbatim;

/// Verify that the assumptions and the conclusion form a valid instance of atomicity assertion ("Verbatim((v1,v2,v3,...,vn,vm)) = Append(Verbatim((v1,v2,v3,...,vn)),Verbatim((vm)))" for any (v1,v2,v3,...,vn) and vm)
pub fn verify_tuple_appendation<'a>(inference: &InferenceInProof<StandardInferenceRule>, style: ExpressionStyle<'a>) -> Result<(),ProofStepSpecificationError<'a>> {
    // Throw an error if there is not exactly one conclusion
    let [conclusion] = *explicit_conclusions_as_sized_slice(inference)?;
    // Throw ane rror if the rule has any assumptions (this rule requires none)
    let [] = *assumptions_as_sized_slice(inference)?;
    
    // Throw an error if there are not three exprs in the conclusion
    let [identity_head, appended, appendation_expr] = *proposition_as_sized_slice(&conclusion)?;
    assert_expression_value(identity_head, BuiltInAtom::Identity.into(), style.clone())?;
    // Throw an error if the appendation component doesn't consist of three components
    let [appendation_head, append_to, to_append] = *expression_as_sized_slice(&appendation_expr)?;
    assert_expression_value(appendation_head, BuiltInAtom::Concatenate.into(), style.clone())?;    
    
    let append_to_verbatim = resolve_verbatim(append_to, style.clone())?;
    let to_append_verbatim = resolve_verbatim(to_append, style.clone())?;
    let appended_verbatim = resolve_verbatim(appended, style.clone())?;

    // Throw an error if the exprs aren't actually identical
    assert_expression_value_equality([append_to_verbatim, to_append_verbatim, appended_verbatim], style)?;
    Ok(())
}

fn resolve_appendation(mut append_to: Vec<Expression>, to_append: &Expression) -> Expression {
    append_to.push(to_append.clone()); Expression::Tuple(append_to)
}
