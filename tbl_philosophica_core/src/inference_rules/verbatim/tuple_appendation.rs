use proof_calculus::{structures::propositions::{ParentOfAssumptions, ParentOfExplicitConclusions}, verification::validity::assertions::as_sized_slice};
use tbl_proof_calculus::{structures::{expressions::{TblExpression, compound::CompoundTblExpression}, proof_calculus_derived::aliases::inferences::{TblInference, TblInferenceRule}}, proofs::assertions::{assert_expression_value, assert_fixed_length_expression_value_equality, expression_as_sized_slice_in_inference, expression_as_slice_in_inference}};

use crate::{inference_rules::verbatim::unwrap_verbatim_expression, structures::atoms::PhilosophicaInferenceAtoms};


#[derive(Clone)]
pub enum TupleAppendationError<C:CompoundTblExpression> {
    WrongAssumptionCount(usize),
    IdentityWrongSize(Option<usize>),
    IdentityWrongHead(TblExpression<C>),
    AppendationWrongSize(Option<usize>),
    AppendationWrongHead(TblExpression<C>),
    PreAppendNotVerbatim(TblExpression<C>),
    ToAppendNotVerbatim(TblExpression<C>),
    PostAppendVerbatim(TblExpression<C>),
    PreAppendAtomic,
    PostAppendAtomic,
    PostAppendNotLengthSuccessorOfPreAppend(usize, usize),
    AppendedNotToAppend(TblExpression<C>, TblExpression<C>)
}

/// Verify that the assumptions and the conclusion form a valid instance of atomicity assertion ("Verbatim((v1,v2,v3,...,vn,vm)) = Append(Verbatim((v1,v2,v3,...,vn)),Verbatim((vm)))" for any (v1,v2,v3,...,vn) and vm)
pub fn verify_tuple_appendation<C: CompoundTblExpression, Rule: TblInferenceRule<C>>(inference: &TblInference<C,Rule>) -> Result<(),TupleAppendationError<C>> {
    // Throw an error if there is not exactly one conclusion
    let [conclusion] = as_sized_slice(inference.get_located_explicit_conclusions())
        .expect("Inference objects must always return exactly one conclusion");
    // Throw ane error if the rule has any assumptions (this rule requires none)
    let [] = as_sized_slice(inference.get_located_assumptions())
        .map_err(|e| TupleAppendationError::WrongAssumptionCount(e.len()))?;
    
    // Throw an error if there are not three exprs in the conclusion
    let [identity_head, post_append_verbatim, appendation_expr] = *expression_as_sized_slice_in_inference(&conclusion)
        .map_err(|e| TupleAppendationError::IdentityWrongSize(e.get_actual_length()))?;
    assert_expression_value(&identity_head, &PhilosophicaInferenceAtoms::Identity.into())
        .map_err(|e| TupleAppendationError::IdentityWrongHead(e.into_expression()))?;
    // Throw an error if the appendation component doesn't consist of three components
    let [appendation_head, pre_append_verbatim, to_append_verbatim] = *expression_as_sized_slice_in_inference(&appendation_expr)
        .map_err(|e| TupleAppendationError::AppendationWrongSize(e.get_actual_length()))?;
    assert_expression_value(&appendation_head, &PhilosophicaInferenceAtoms::Concatenate.into())
        .map_err(|e| TupleAppendationError::AppendationWrongHead(e.into_expression()))?;    
    
    // Extract the verbatim expressions, throwing an error one of the expressions has no verbatim component
    let pre_append = unwrap_verbatim_expression(&pre_append_verbatim)
        .map_err(|e| TupleAppendationError::PreAppendNotVerbatim(e.expression().obj))?;
    let to_append = unwrap_verbatim_expression(&to_append_verbatim)
        .map_err(|e| TupleAppendationError::ToAppendNotVerbatim(e.expression().obj))?;
    let post_append = unwrap_verbatim_expression(&post_append_verbatim)
        .map_err(|e| TupleAppendationError::PostAppendVerbatim(e.expression().obj))?;
    
    // Convert append_to and appended to vecs
    let pre_append_vec = expression_as_slice_in_inference(&pre_append)
        .map_err(|_| TupleAppendationError::PreAppendAtomic)?;
    let post_append_vec = expression_as_slice_in_inference(&post_append)
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
            TupleAppendationError::AppendedNotToAppend(to_append.obj, appended.obj)
        })?;

    Ok(())
}
