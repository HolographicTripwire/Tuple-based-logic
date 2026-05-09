use crate::expressions::{
    paths::TblSubexpressionInExpressionPath,
    types::{
        assigned::compound::TblExpressionCompound,
        unassigned::at_path_enum::{
            OwnedUnassignedTblExpressionAtPathEnum, UnassignedTblExpressionAtPathEnum,
        },
    },
};

pub type UnassignedTblSubexpressionInExpressionEnum<'a, C: TblExpressionCompound> =
    UnassignedTblExpressionAtPathEnum<'a, C, TblSubexpressionInExpressionPath>;
pub type OwnedUnassignedTblSubexpressionInExpressionEnum<C: TblExpressionCompound> =
    OwnedUnassignedTblExpressionAtPathEnum<C, TblSubexpressionInExpressionPath>;
