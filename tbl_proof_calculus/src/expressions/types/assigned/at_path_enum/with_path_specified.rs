use crate::expressions::{paths::TblSubexpressionInExpressionPath, types::assigned::{at_path_enum::{OwnedTblExpressionAtPathEnum, TblExpressionAtPathEnum}, compound::CompoundTblExpression}};

pub type TblSubexpressionInExpressionEnum<'a,C: CompoundTblExpression> = TblExpressionAtPathEnum<'a,C,TblSubexpressionInExpressionPath>;
pub type OwnedTblSubexpressionInExpressionEnum<C: CompoundTblExpression> = OwnedTblExpressionAtPathEnum<C,TblSubexpressionInExpressionPath>;
