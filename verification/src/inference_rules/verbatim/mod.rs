mod atomicity_assertion;
mod atom_differentiation;
mod tuple_appendation;

pub use atomicity_assertion::verify_atomicity_assertion;
pub use atom_differentiation::verify_atom_differentiation;
use tbl_structures::{atoms::BuiltInAtom, inference::path::SubexpressionInInference};
pub use tuple_appendation::verify_tuple_appendation;

use crate::{inference_rules::{assertions::{assert_expression_value, expression_as_sized_slice}, error::ProofStepSpecificationError}};

/// Take an expression, and if it is in the form (Verbatim, x) return x, otherwise return an Error
fn resolve_verbatim<'a>(verbatim_expr: &'a SubexpressionInInference) -> Result<SubexpressionInInference<'a>,ProofStepSpecificationError>{
    let [verbatim_head, verbatim_tail] = *expression_as_sized_slice(verbatim_expr)?;
    assert_expression_value(&verbatim_head, &BuiltInAtom::Verbatim.into())?;
    Ok(verbatim_tail)
}
