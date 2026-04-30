use crate::expressions::{paths::TblSubexpressionInExpressionPath, types::{assigned::compound::CompoundTblExpression, unassigned::at_path_enum::{OwnedUnassignedTblExpressionAtPathEnum, UnassignedTblExpressionAtPathEnum}}};

pub type UnassignedTblSubexpressionInExpressionEnum<'a,C: CompoundTblExpression> = UnassignedTblExpressionAtPathEnum<'a,C,TblSubexpressionInExpressionPath>;
pub type OwnedUnassignedTblSubexpressionInExpressionEnum<C: CompoundTblExpression> = OwnedUnassignedTblExpressionAtPathEnum<C,TblSubexpressionInExpressionPath>;
