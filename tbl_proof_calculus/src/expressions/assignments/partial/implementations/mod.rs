use itertools::Itertools;
use proof_calculus::utils::{
    collections::maps::KeyConflictError,
    traits::map::{Map, MapWithoutConflicts},
};

use crate::{
    expressions::{
        assignments::partial::errors::{
            assignment::TblPartialAssignmentError,
            reverse_assignment::{
                TblPartialReverseAssignmentError, TblPartialReverseAssignmentVariableConflictError,
            },
        },
        paths::{
            TblSubexpressionInExpressionPath, immediate::ImmediateTblSubexpressionInExpressionPath,
        },
        types::unassigned::{
            UnassignedTblExpression, compound::UnassignedTblExpressionCompound,
            variable::TblExpressionVariable,
        },
    },
    proof_calculus_derived::aliases::propositions::types::unassigned::UnassignedTblProposition,
};

pub mod btree;
pub mod dense;
pub mod hash;

trait TblPartialAssignmentHelper<Uc: UnassignedTblExpressionCompound>: Default {
    fn get(&self, var: &TblExpressionVariable) -> Option<&UnassignedTblExpression<Uc>>;
    fn insert(
        &mut self,
        var: TblExpressionVariable,
        expr: UnassignedTblExpression<Uc>,
    ) -> Result<(), KeyConflictError<TblExpressionVariable, UnassignedTblExpression<Uc>>>;

    fn partial_assign_helper<
        PreUc: UnassignedTblExpressionCompound,
        PostUc: UnassignedTblExpressionCompound,
    >(
        &self,
        unassigned: &UnassignedTblExpression<PreUc>,
    ) -> Result<UnassignedTblExpression<PostUc>, TblPartialAssignmentError>
    where
        PostUc: for<'a> From<&'a Uc>
            + for<'a> From<&'a PreUc>
            + FromIterator<UnassignedTblExpression<PostUc>>,
    {
        match unassigned {
            UnassignedTblExpression::Atom(atom) => Ok(UnassignedTblExpression::Atom(*atom)),
            UnassignedTblExpression::Variable(variable) => match self.get(variable) {
                Some(expr) => Ok(expr.into()),
                None => Err(TblPartialAssignmentError(*variable)),
            },
            UnassignedTblExpression::Compound(compound) => Ok(UnassignedTblExpression::Compound(
                compound
                    .get_immediate_subexpressions()
                    .into_iter()
                    .map(|uexpr| {
                        self.partial_assign_helper(uexpr)
                            .map(|v| UnassignedTblExpression::<PostUc>::from(v))
                    })
                    .try_collect()?,
            )),
        }
    }

    fn partial_reverse_assign_helper<
        PreUc: UnassignedTblExpressionCompound,
        PostUc: UnassignedTblExpressionCompound,
    >(
        unassigned: &UnassignedTblExpression<PreUc>,
        assigned: &UnassignedTblExpression<PostUc>,
    ) -> Result<Self, TblPartialReverseAssignmentError<Uc>>
    where
        Uc: for<'a> From<&'a PostUc>,
        PostUc: for<'a> From<&'a Uc> + FromIterator<UnassignedTblExpression<PostUc>>,
    {
        let mut assignments = Self::default();
        let mut path = TblSubexpressionInExpressionPath::default();
        Self::partial_reverse_assignment_helper_inner(
            unassigned,
            assigned,
            &mut assignments,
            &mut path,
        )
        .map(|_| assignments)
    }
    fn partial_reverse_assignment_helper_inner<
        PreUc: UnassignedTblExpressionCompound,
        PostUc: UnassignedTblExpressionCompound
            + for<'a> From<&'a Uc>
            + FromIterator<UnassignedTblExpression<PostUc>>,
    >(
        unassigned: &UnassignedTblExpression<PreUc>,
        assigned: &UnassignedTblExpression<PostUc>,
        assignments: &mut Self,
        current_path: &mut TblSubexpressionInExpressionPath,
    ) -> Result<(), TblPartialReverseAssignmentError<Uc>>
    where
        Uc: for<'a> From<&'a PostUc>,
    {
        match (unassigned, assigned) {
            // If both are atoms, assert that the values of the atoms are equal
            (UnassignedTblExpression::Atom(unassigned_atom), subsumed_expression) => {
                if &UnassignedTblExpression::<PostUc>::Atom(*unassigned_atom) != subsumed_expression
                {
                    Err(TblPartialReverseAssignmentError::atom_value_conflict(
                        *unassigned_atom,
                        subsumed_expression.into(),
                        current_path.clone(),
                    ))
                } else {
                    Ok(())
                }
            } // If both are compounds, recurse
            (
                UnassignedTblExpression::Compound(unassigned_compound),
                UnassignedTblExpression::Compound(assigned_compound),
            ) => {
                // Assert that the lengths of the two compounds are equal
                let unassigned_length = unassigned_compound.len();
                let assigned_length = assigned_compound.len();
                if unassigned_length != assigned_length {
                    return Err(TblPartialReverseAssignmentError::compound_length_conflict(
                        unassigned_length,
                        assigned_length,
                        current_path.clone(),
                    ));
                }
                // Recurse, performing reverse_assign_helper on all subexpressions
                let immediate_subpaths =
                    (0..assigned_length).map(|i| ImmediateTblSubexpressionInExpressionPath(i));
                for ((unassigned_subexpression, assigned_expression), immediate_subpath) in
                    unassigned_compound
                        .as_slice()
                        .iter()
                        .zip(assigned_compound.as_slice().iter())
                        .zip(immediate_subpaths)
                {
                    current_path.0.push(immediate_subpath);
                    Self::partial_reverse_assignment_helper_inner(
                        unassigned_subexpression,
                        assigned_expression,
                        assignments,
                        current_path,
                    )?;
                    current_path.0.pop();
                }
                // If successful, return Ok
                Ok(())
            } // If the unassigned value is a variable, insert into the assignments being constructed
            (UnassignedTblExpression::Variable(unassigned_variable), expr) => assignments
                .insert(*unassigned_variable, expr.into())
                .map_err(|conflict| {
                    TblPartialReverseAssignmentError::VariableConflict(
                        TblPartialReverseAssignmentVariableConflictError(conflict),
                    )
                }),
            (
                UnassignedTblExpression::Compound(_),
                UnassignedTblExpression::Atom(assigned_atom),
            ) => Err(
                TblPartialReverseAssignmentError::compound_matched_with_atom(
                    *assigned_atom,
                    current_path.clone(),
                ),
            ),
            (
                UnassignedTblExpression::Compound(_),
                UnassignedTblExpression::Variable(assigned_variable),
            ) => Err(
                TblPartialReverseAssignmentError::compound_matched_with_variable(
                    *assigned_variable,
                    current_path.clone(),
                ),
            ),
        }
    }
}

impl<
    C: UnassignedTblExpressionCompound,
    M: Default + MapWithoutConflicts<TblExpressionVariable, UnassignedTblProposition<C>>,
> TblPartialAssignmentHelper<C> for M
{
    fn get(&self, var: &TblExpressionVariable) -> Option<&UnassignedTblExpression<C>> {
        Map::get(self, var)
    }

    fn insert(
        &mut self,
        var: TblExpressionVariable,
        expr: UnassignedTblExpression<C>,
    ) -> Result<(), KeyConflictError<TblExpressionVariable, UnassignedTblExpression<C>>> {
        Self::insert_conflictless(self, var, expr)
    }
}
