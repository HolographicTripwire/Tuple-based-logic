use crate::{expressions::unassigned::at_path_enum::{OwnedUnassignedTblExpressionAtPathEnum, UnassignedTblExpressionAtPathEnum}, expressions::assigned::{compound::CompoundTblExpression, subexpressions::TblSubexpressionInExpressionPath}};

pub type UnassignedTblSubexpressionInExpressionEnum<'a,C: CompoundTblExpression> = UnassignedTblExpressionAtPathEnum<'a,C,TblSubexpressionInExpressionPath>;
pub type OwnedUnassignedTblSubexpressionInExpressionEnum<C: CompoundTblExpression> = OwnedUnassignedTblExpressionAtPathEnum<C,TblSubexpressionInExpressionPath>;
