use crate::expressions::{
    paths::TblSubexpressionInExpressionPath,
    types::assigned::{
        at_path_enum::{OwnedTblExpressionAtPathEnum, TblExpressionAtPathEnum},
        compound::TblExpressionCompound,
    },
};

pub type TblSubexpressionInExpressionEnum<'a, C: TblExpressionCompound> =
    TblExpressionAtPathEnum<'a, C, TblSubexpressionInExpressionPath>;
pub type OwnedTblSubexpressionInExpressionEnum<C: TblExpressionCompound> =
    OwnedTblExpressionAtPathEnum<C, TblSubexpressionInExpressionPath>;
