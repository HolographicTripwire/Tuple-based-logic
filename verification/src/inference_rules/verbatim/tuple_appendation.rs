use tbl_structures::{atoms::BuiltInAtom, expressions::Expression, inference::{Inference, InferenceRule}, path_composites::{ExpressionInInference, OwnedExpressionInProof}, proof::InferenceInProof};
use tbl_textualization::structures::expressions::ExpressionStyle;


use crate::{assertions::*, inference_rules::StandardInferenceRule};

use super::resolve_verbatim;

#[derive(Clone)]
pub enum TupleAppendationError {
    WrongExplicitConclusionCount(usize),
    WrongAssumptionCount(usize),
    IdentityWrongSize(Option<usize>),
    IdentityWrongHead(Expression),
    AppendationWrongSize(Option<usize>),
    AppendationWrongHead(Expression),
    AppendToNotVerbatim(Expression),
    ToAppendNotVerbatim(Expression),
    AppendedNotVerbatim(Expression),
    AppendToAtomic,
    AppendedAtomic
}

/// Verify that the assumptions and the conclusion form a valid instance of atomicity assertion ("Verbatim((v1,v2,v3,...,vn,vm)) = Append(Verbatim((v1,v2,v3,...,vn)),Verbatim((vm)))" for any (v1,v2,v3,...,vn) and vm)
pub fn verify_tuple_appendation<Rule: InferenceRule>(inference: &Inference<Rule>) -> Result<(),TupleAppendationError> {
    // Throw an error if there is not exactly one conclusion
    let [conclusion] = *explicit_conclusions_as_sized_slice(inference)
        .map_err(|e| TupleAppendationError::WrongExplicitConclusionCount(e.get_actual_count()))?;
    // Throw ane rror if the rule has any assumptions (this rule requires none)
    let [] = *assumptions_as_sized_slice(inference)
        .map_err(|e| TupleAppendationError::WrongAssumptionCount(e.get_actual_count()))?;
    
    // Throw an error if there are not three exprs in the conclusion
    let [identity_head, appended, appendation_expr] = *proposition_as_sized_slice(&conclusion)
        .map_err(|e| TupleAppendationError::IdentityWrongSize(e.get_actual_length()))?;
    assert_expression_value(&identity_head, &BuiltInAtom::Identity.into())
        .map_err(|e| TupleAppendationError::IdentityWrongHead(e.into_expression()))?;
    // Throw an error if the appendation component doesn't consist of three components
    let [appendation_head, append_to, to_append] = *expression_as_sized_slice(&appendation_expr)
        .map_err(|e| TupleAppendationError::AppendationWrongSize(e.get_actual_length()))?;
    assert_expression_value(&appendation_head, &BuiltInAtom::Concatenate.into())
        .map_err(|e| TupleAppendationError::AppendationWrongHead(e.into_expression()))?;    
    
    // Extract the verbatim expressions, throwing an error one of the expressions has no verbatim component
    let append_to_verbatim = resolve_verbatim(append_to)
        .map_err(|e| TupleAppendationError::AppendToNotVerbatim(e.value))?;
    let to_append_verbatim = resolve_verbatim(to_append)
        .map_err(|e| TupleAppendationError::ToAppendNotVerbatim(e.value))?;
    let appended_verbatim = resolve_verbatim(appended)
        .map_err(|e| TupleAppendationError::AppendedNotVerbatim(e.value))?;
    
    // Compare the three verbatim expressions
    let vec1 = expression_as_slice(&append_to_verbatim)
        .map_err(|e| ExpressionLengthSuccessorError::AppendToAtomic)?;
    let vec2 = expression_as_slice(&appended_verbatim)
        .map_err(|e| ExpressionLengthSuccessorError::AppendToAtomic)?;
    
    Ok(())
}
