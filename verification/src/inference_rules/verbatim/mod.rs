mod atomicity_assertion;
mod atom_differentiation;
mod tuple_appendation;

pub use atomicity_assertion::verify_atomicity_assertion;
pub use atom_differentiation::verify_atom_differentiation;
use tbl_structures::{atoms::BuiltInAtom, path_composites::OwnedExpressionInProof};
use tbl_textualization::structures::expressions::ExpressionStyle;
pub use tuple_appendation::verify_tuple_appendation;

use crate::{assertions::*, errors::specification_error::ProofStepSpecificationError};

/// Take an expression, and if it is in the form (Verbatim, x) return x, otherwise return an Error
fn resolve_verbatim<'a>(verbatim_expr: OwnedExpressionInProof, style: ExpressionStyle<'a>) -> Result<OwnedExpressionInProof,ProofStepSpecificationError<'a>>{
    let [verbatim_head, verbatim_tail] = *expression_as_sized_slice(&verbatim_expr)?;
    assert_expression_value(verbatim_head, BuiltInAtom::Verbatim.into(), style)?;
    Ok(verbatim_tail)
}
