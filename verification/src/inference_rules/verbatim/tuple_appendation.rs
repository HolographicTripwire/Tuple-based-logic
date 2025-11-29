use tbl_structures::{atoms::BuiltInAtom, expressions::Expression, path_composites::OwnedExpressionInProof, proof::InferenceInProof};
use tbl_textualization::structures::expressions::ExpressionStyle;


use crate::{assertions::*, errors::specification_error::{Assessor, ProofStepSpecificationError}, inference_rules::StandardInferenceRule};

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
    
    // Extract the verbatim expressions, throwing an error one of the expressions has no verbatim component
    let append_to_verbatim = resolve_verbatim(append_to, style.clone())?;
    let to_append_verbatim = resolve_verbatim(to_append, style.clone())?;
    let appended_verbatim = resolve_verbatim(appended, style.clone())?;
    
    // Compare the three verbatim expressions
    expression_length_successor_predicate((append_to_verbatim,appended_verbatim))

    Ok(())
}

enum ExpressionLengthSuccessorError {
    AppendToAtomic,
    AppendedAtomic,
    WrongLength
}

/// Get a [Predicate](NaryPredicate) which takes n [Expressions](OwnedExpressionInProof) and checks if their lengths are equal
pub fn expression_length_successor_predicate<'a,const N: usize>() -> impl Assessor<'a,(OwnedExpressionInProof,OwnedExpressionInProof),ExpressionLengthSuccessorError> {
    move |(append_to,appended)| { 
        let Ok(append_to_exprs) = append_to.0.obj().as_slice() else { return Err(ExpressionLengthSuccessorError::AppendToAtomic) };
        let Ok(appended_exprs) = appended.0.obj().as_slice() else { return Err(ExpressionLengthSuccessorError::AppendedAtomic) };
        
    }
}
/// Get a [Stringifier](NaryStringifier) which takes an [Expressions](OwnedExpressionInProof) and returns an error message saying that their lengths aren't equal
pub fn expression_length_equality_stringifier<'a, const N: usize>() -> impl NaryStringifier<'a,(OwnedExpressionInProof,OwnedExpressionInProof)> {
    move || format!(
        "Expression lengths expected to be equal, but weren't; {atomicities}",
        atomicities = os.map(|o| 
            o.0.path().to_string()
            + " -> " +
            &stringify_length(o.0.obj())
        ).join(", ")
    )
}
/// Get a [Checker](StringifiablePredicate) which takes n [Expressions](OwnedExpressionInProof) and returns an error message if their lengths aren't equal
pub fn expression_length_equality_check<'a, const N: usize>() -> StringifiablePredicate<'a,[OwnedExpressionInProof;N]> { StringifiablePredicate::new(
    expression_length_equality_predicate(),
    expression_length_equality_stringifier(),
)}

/// Check that the provided [Expressions](OwnedExpressionInProof) have equal length, returning an error otherwise
pub fn assert_expression_length_equality<'a,const N: usize>(exprs: [OwnedExpressionInProof; N]) -> Result<(), ProofStepSpecificationError<'a>> {
    expression_length_equality_check().evaluate(exprs)
        .map_err(|assertion| ProofStepSpecificationError::from_inner(assertion))
}
