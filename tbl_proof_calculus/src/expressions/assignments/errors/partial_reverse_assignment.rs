use path_lib::obj_at_path::{ObjAtPath, OwnedObjAtPath};
use proof_calculus::utils::collections::maps::conflictless::KeyConflictError;

use crate::expressions::{
    paths::TblSubexpressionInExpressionPath,
    types::{
        assigned::{atom::TblExpressionAtom, subexpressions::OwnedTblExpressionAtomInExpression},
        unassigned::{
            UnassignedTblExpression, compound::UnassignedTblExpressionCompound,
            subexpressions::OwnedTblExpressionVariableInExpression,
            variable::TblExpressionVariable,
        },
    },
};

pub struct TblPartialReverseAssignmentAtomValueError<Uc: UnassignedTblExpressionCompound> {
    pub unassigned_atom: TblExpressionAtom,
    pub assigned_expression: UnassignedTblExpression<Uc>,
}
pub type TblReverseAssignmentAtomValueConflictErrorInExpression<
    'a,
    Uc: UnassignedTblExpressionCompound,
> = ObjAtPath<'a, TblPartialReverseAssignmentAtomValueError<Uc>, TblSubexpressionInExpressionPath>;
pub type OwnedTblReverseAssignmentAtomValueConflictErrorInExpression<
    Uc: UnassignedTblExpressionCompound,
> = OwnedObjAtPath<TblPartialReverseAssignmentAtomValueError<Uc>, TblSubexpressionInExpressionPath>;

pub struct TblPartialReverseAssignmentCompoundLengthConflictError {
    pub unassigned_length: usize,
    pub assigned_length: usize,
}
pub type TblPartialReverseAssignmentCompoundLengthConflictErrorInExpression<'a> = ObjAtPath<
    'a,
    TblPartialReverseAssignmentCompoundLengthConflictError,
    TblSubexpressionInExpressionPath,
>;
pub type OwnedTblPartialReverseAssignmentCompoundLengthConflictErrorInExpression = OwnedObjAtPath<
    TblPartialReverseAssignmentCompoundLengthConflictError,
    TblSubexpressionInExpressionPath,
>;

pub struct TblPartialReverseAssignmentVariableConflictError<Uc: UnassignedTblExpressionCompound>(
    pub KeyConflictError<TblExpressionVariable, UnassignedTblExpression<Uc>>,
);

pub enum TblPartialReverseAssignmentError<Uc: UnassignedTblExpressionCompound> {
    AtomValueInequal(OwnedTblReverseAssignmentAtomValueConflictErrorInExpression<Uc>),
    CompoundMatchedWithAtom(OwnedTblExpressionAtomInExpression),
    CompoundMatchedWithVariable(OwnedTblExpressionVariableInExpression),
    CompoundLengthConflict(OwnedTblPartialReverseAssignmentCompoundLengthConflictErrorInExpression),
    VariableConflict(TblPartialReverseAssignmentVariableConflictError<Uc>),
}
impl<Uc: UnassignedTblExpressionCompound> TblPartialReverseAssignmentError<Uc> {
    pub fn atom_value_conflict(
        unassigned_atom: TblExpressionAtom,
        assigned_expression: UnassignedTblExpression<Uc>,
        path: TblSubexpressionInExpressionPath,
    ) -> Self {
        Self::AtomValueInequal(OwnedObjAtPath {
            obj: TblPartialReverseAssignmentAtomValueError {
                unassigned_atom,
                assigned_expression,
            },
            path,
        })
    }
    pub fn compound_matched_with_atom(
        assigned_atom: TblExpressionAtom,
        path: TblSubexpressionInExpressionPath,
    ) -> Self {
        Self::CompoundMatchedWithAtom(OwnedObjAtPath {
            obj: assigned_atom,
            path,
        })
    }
    pub fn compound_matched_with_variable(
        assigned_variable: TblExpressionVariable,
        path: TblSubexpressionInExpressionPath,
    ) -> Self {
        Self::CompoundMatchedWithVariable(OwnedObjAtPath {
            obj: assigned_variable,
            path,
        })
    }
    pub fn compound_length_conflict(
        unassigned_length: usize,
        assigned_length: usize,
        path: TblSubexpressionInExpressionPath,
    ) -> Self {
        Self::CompoundLengthConflict(OwnedObjAtPath {
            obj: TblPartialReverseAssignmentCompoundLengthConflictError {
                unassigned_length,
                assigned_length,
            },
            path,
        })
    }
    pub fn variable_conflict(
        conflict: TblPartialReverseAssignmentVariableConflictError<Uc>,
    ) -> Self {
        Self::VariableConflict(conflict)
    }
}
