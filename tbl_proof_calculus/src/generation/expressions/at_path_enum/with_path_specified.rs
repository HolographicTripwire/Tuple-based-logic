use crate::{generation::expressions::at_path_enum::{OwnedUnassignedTblExpressionAtPathEnum, UnassignedTblExpressionAtPathEnum}, structures::expressions::{compound::CompoundTblExpression, subexpressions::TblSubexpressionInExpressionPath}};

pub type UnassignedTblSubexpressionInExpressionEnum<'a,C: CompoundTblExpression> = UnassignedTblExpressionAtPathEnum<'a,C,TblSubexpressionInExpressionPath>;
pub type OwnedUnassignedTblSubexpressionInExpressionEnum<C: CompoundTblExpression> = OwnedUnassignedTblExpressionAtPathEnum<C,TblSubexpressionInExpressionPath>;
