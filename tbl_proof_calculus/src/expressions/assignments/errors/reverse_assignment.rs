use path_lib::obj_at_path::{ObjAtPath, OwnedObjAtPath};
use proof_calculus::utils::collections::maps::KeyConflictError;

use crate::expressions::{paths::TblSubexpressionInExpressionPath, types::{assigned::{TblExpression, atom::TblExpressionAtom, compound::TblExpressionCompound, subexpressions::OwnedTblExpressionAtomInExpression}, unassigned::variable::TblExpressionVariable}};

pub struct TblReverseAssignmentAtomValueError<C: TblExpressionCompound>{
    pub unassigned_atom: TblExpressionAtom,
    pub assigned_expression: TblExpression<C>
}
pub type TblReverseAssignmentAtomValueConflictErrorInExpression<'a,C: TblExpressionCompound> = ObjAtPath<'a,TblReverseAssignmentAtomValueError<C>,TblSubexpressionInExpressionPath>;
pub type OwnedTblReverseAssignmentAtomValueConflictErrorInExpression<C: TblExpressionCompound> = OwnedObjAtPath<TblReverseAssignmentAtomValueError<C>,TblSubexpressionInExpressionPath>;

pub struct TblReverseAssignmentCompoundLengthConflictError {
    pub unassigned_length: usize,
    pub assigned_length: usize,
}
pub type TblReverseAssignmentCompoundLengthConflictErrorInExpression<'a> = ObjAtPath<'a,TblReverseAssignmentCompoundLengthConflictError,TblSubexpressionInExpressionPath>;
pub type OwnedTblReverseAssignmentCompoundLengthConflictErrorInExpression = OwnedObjAtPath<TblReverseAssignmentCompoundLengthConflictError,TblSubexpressionInExpressionPath>;

pub struct TblReverseAssignmentVariableConflictError<C: TblExpressionCompound>(pub KeyConflictError<TblExpressionVariable,TblExpression<C>>);

pub enum TblReverseAssignmentError<C: TblExpressionCompound> {
    AtomValueInequal(OwnedTblReverseAssignmentAtomValueConflictErrorInExpression<C>),
    CompoundMatchedWithAtom(OwnedTblExpressionAtomInExpression),
    CompoundLengthConflict(OwnedTblReverseAssignmentCompoundLengthConflictErrorInExpression),
    VariableConflict(TblReverseAssignmentVariableConflictError<C>)
}
impl <C: TblExpressionCompound> TblReverseAssignmentError<C> {
    pub fn atom_value_conflict(unassigned_atom: TblExpressionAtom, assigned_expression: TblExpression<C>, path: TblSubexpressionInExpressionPath) -> Self
        { Self::AtomValueInequal(OwnedObjAtPath { obj: TblReverseAssignmentAtomValueError { unassigned_atom, assigned_expression }, path }) }
    pub fn compound_matched_with_atom(assigned_atom: TblExpressionAtom, path: TblSubexpressionInExpressionPath) -> Self
        { Self::CompoundMatchedWithAtom(OwnedObjAtPath { obj: assigned_atom, path }) }
    pub fn compound_length_conflict(unassigned_length: usize, assigned_length: usize, path: TblSubexpressionInExpressionPath) -> Self
        { Self::CompoundLengthConflict(OwnedObjAtPath { obj: TblReverseAssignmentCompoundLengthConflictError { unassigned_length, assigned_length }, path }) }
    pub fn variable_conflict(conflict: TblReverseAssignmentVariableConflictError<C>) -> Self
        { Self::VariableConflict(conflict) }
}
