use tbl_structures::{atoms::BuiltInAtom, expressions::Expression, inference::{Inference, InferenceRule}};

use crate::{assertions::*, inference_rules::verbatim::unwrap_verbatim_expression};

#[derive(Clone)]
pub enum TupleAppendationError {
    WrongExplicitConclusionCount(usize),
    WrongAssumptionCount(usize),
    IdentityWrongSize(Option<usize>),
    IdentityWrongHead(Expression),
    AppendationWrongSize(Option<usize>),
    AppendationWrongHead(Expression),
    PreAppendNotVerbatim(Expression),
    ToAppendNotVerbatim(Expression),
    PostAppendVerbatim(Expression),
    PreAppendAtomic,
    PostAppendAtomic,
    PostAppendNotLengthSuccessorOfPreAppend(usize, usize),
    AppendedNotToAppend(Expression, Expression)
}

/// Verify that the assumptions and the conclusion form a valid instance of atomicity assertion ("Verbatim((v1,v2,v3,...,vn,vm)) = Append(Verbatim((v1,v2,v3,...,vn)),Verbatim((vm)))" for any (v1,v2,v3,...,vn) and vm)
pub fn verify_tuple_appendation<Rule: InferenceRule>(inference: &Inference<Rule>) -> Result<(),TupleAppendationError> {
    // Throw an error if there is not exactly one conclusion
    let [conclusion] = *explicit_conclusions_as_sized_slice(inference)
        .map_err(|e| TupleAppendationError::WrongExplicitConclusionCount(e.get_actual_count()))?;
    // Throw ane error if the rule has any assumptions (this rule requires none)
    let [] = *assumptions_as_sized_slice(inference)
        .map_err(|e| TupleAppendationError::WrongAssumptionCount(e.get_actual_count()))?;
    
    // Throw an error if there are not three exprs in the conclusion
    let [identity_head, post_append_verbatim, appendation_expr] = *proposition_as_sized_slice(&conclusion)
        .map_err(|e| TupleAppendationError::IdentityWrongSize(e.get_actual_length()))?;
    assert_expression_value(&identity_head, &BuiltInAtom::Identity.into())
        .map_err(|e| TupleAppendationError::IdentityWrongHead(e.into_expression()))?;
    // Throw an error if the appendation component doesn't consist of three components
    let [appendation_head, pre_append_verbatim, to_append_verbatim] = *expression_as_sized_slice(&appendation_expr)
        .map_err(|e| TupleAppendationError::AppendationWrongSize(e.get_actual_length()))?;
    assert_expression_value(&appendation_head, &BuiltInAtom::Concatenate.into())
        .map_err(|e| TupleAppendationError::AppendationWrongHead(e.into_expression()))?;    
    
    // Extract the verbatim expressions, throwing an error one of the expressions has no verbatim component
    let pre_append = unwrap_verbatim_expression(&pre_append_verbatim)
        .map_err(|_| TupleAppendationError::PreAppendNotVerbatim(pre_append_verbatim.obj().clone()))?;
    let to_append = unwrap_verbatim_expression(&to_append_verbatim)
        .map_err(|_| TupleAppendationError::ToAppendNotVerbatim(to_append_verbatim.obj().clone()))?;
    let post_append = unwrap_verbatim_expression(&post_append_verbatim)
        .map_err(|_| TupleAppendationError::PostAppendVerbatim(post_append_verbatim.obj().clone()))?;
    
    // Convert append_to and appended to vecs
    let pre_append_vec = expression_as_slice(&pre_append)
        .map_err(|_| TupleAppendationError::PreAppendAtomic)?;
    let post_append_vec = expression_as_slice(&post_append)
        .map_err(|_| TupleAppendationError::PostAppendAtomic)?;
    
    // Check that appended has one more element than eppend_to
    let pre_append_length = pre_append_vec.len();
    let post_append_length = post_append_vec.len();
    if pre_append_length != post_append_length + 1 { return Err(TupleAppendationError::PostAppendNotLengthSuccessorOfPreAppend(pre_append_length, post_append_length)) }
    // Check that appended element is equal to to_append element
    let appended = post_append_vec.last().expect("post_append had no final element even though it had length greater than 0");
    assert_fixed_length_expression_value_equality(&[&to_append, &appended])
        .map_err(|e| { 
            let [to_append, appended] = e.expressions;
            TupleAppendationError::AppendedNotToAppend(to_append.into_obj_and_path().0, appended.into_obj_and_path().0)
        })?;

    Ok(())
}
