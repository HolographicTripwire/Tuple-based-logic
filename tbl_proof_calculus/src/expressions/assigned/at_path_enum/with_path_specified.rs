use crate::expressions::assigned::{at_path_enum::{OwnedTblExpressionAtPathEnum, TblExpressionAtPathEnum}, compound::CompoundTblExpression, subexpressions::TblSubexpressionInExpressionPath};

pub type TblSubexpressionInExpressionEnum<'a,C: CompoundTblExpression> = TblExpressionAtPathEnum<'a,C,TblSubexpressionInExpressionPath>;
pub type OwnedTblSubexpressionInExpressionEnum<C: CompoundTblExpression> = OwnedTblExpressionAtPathEnum<C,TblSubexpressionInExpressionPath>;
