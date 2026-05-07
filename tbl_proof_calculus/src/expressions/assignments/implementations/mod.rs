use itertools::Itertools;
use proof_calculus::utils::collections::maps::KeyConflictError;

use crate::expressions::{assignments::errors::{assignment::TblAssignmentError, reverse_assignment::{TblReverseAssignmentError, TblReverseAssignmentVariableConflictError}}, paths::{TblSubexpressionInExpressionPath, immediate::ImmediateTblSubexpressionInExpressionPath}, types::{assigned::{TblExpression, compound::TblExpressionCompound}, unassigned::{UnassignedTblExpression, compound::UnassignedTblExpressionCompound, variable::TblExpressionVariable}}};

pub mod dense;
pub mod sparse;

trait TblAssignmentHelper<C: TblExpressionCompound>: Default {
    fn get(&self, var: &TblExpressionVariable) -> Option<&TblExpression<C>>;
    fn insert(&mut self, var: TblExpressionVariable, expr: TblExpression<C>) -> Result<(),KeyConflictError<TblExpressionVariable,TblExpression<C>>>;

    fn assign_helper<PreUc: UnassignedTblExpressionCompound, PostC: TblExpressionCompound>
    (&self, subsuming_uprop: &UnassignedTblExpression<PreUc>) -> Result<TblExpression<PostC>, TblAssignmentError>
    where PostC: for<'a> From<&'a C> + for<'a> From<&'a PreUc> + FromIterator<TblExpression<PostC>>{
        match subsuming_uprop {
            UnassignedTblExpression::Atom(atom) => Ok(TblExpression::Atom(*atom)),
            UnassignedTblExpression::Variable(variable) => match self.get(variable) {
                Some(expr) => Ok(expr.into()),
                None => Err(TblAssignmentError(*variable)),
            }, UnassignedTblExpression::Compound(compound) => Ok(TblExpression::Compound(
                compound.get_immediate_subexpressions().into_iter()
                    .map(|uexpr| self.assign_helper(uexpr).map(|v| TblExpression::<PostC>::from(v)) )
                    .try_collect()?
            ))
        }
    }

    fn reverse_assign_helper<PreUc: UnassignedTblExpressionCompound, PostC: TblExpressionCompound + for<'a> From<&'a C> + FromIterator<TblExpression<PostC>>>
    (pre_assignment_uprop: &UnassignedTblExpression<PreUc>, post_assignment_prop: &TblExpression<PostC>) -> Result<Self,TblReverseAssignmentError<C>>
    where C: for<'a> From<&'a PostC> {
        let mut assignments = Self::default();
        let mut path = TblSubexpressionInExpressionPath::default();
        Self::reverse_assignment_helper_inner(pre_assignment_uprop, post_assignment_prop, &mut assignments, &mut path)
            .map(|_| assignments)
    }
    fn reverse_assignment_helper_inner<PreUc: UnassignedTblExpressionCompound, PostC: TblExpressionCompound + for<'a> From<&'a C> + FromIterator<TblExpression<PostC>>>
    (pre_assignment_uprop: &UnassignedTblExpression<PreUc>, post_assignment_prop: &TblExpression<PostC>, assignments: &mut Self, current_path: &mut TblSubexpressionInExpressionPath)
    -> Result<(),TblReverseAssignmentError<C>> where C: for<'a> From<&'a PostC> {
        match (pre_assignment_uprop, post_assignment_prop) {
            // If both are atoms, assert that the values of the atoms are equal
            (UnassignedTblExpression::Atom(unassigned_atom), assigned_expression) => {
                if &TblExpression::<PostC>::Atom(*unassigned_atom) != assigned_expression { Err(TblReverseAssignmentError::atom_value_conflict(
                    *unassigned_atom, 
                    assigned_expression.into(),
                    current_path.clone()
                )) } else { Ok(()) }
            }, // If both are compounds, recurse
            (UnassignedTblExpression::Compound(unassigned_compound), TblExpression::Compound(assigned_compound)) => {
                // Assert that the lengths of the two compounds are equal
                let unassigned_length = unassigned_compound.len();
                let assigned_length = assigned_compound.len();
                if unassigned_length != assigned_length
                    { return Err(TblReverseAssignmentError::compound_length_conflict(unassigned_length,assigned_length,current_path.clone())) }
                // Recurse, performing reverse_assign_helper on all subexpressions
                let immediate_subpaths = (0..assigned_length).map(|i| ImmediateTblSubexpressionInExpressionPath(i));
                for ((unassigned_subexpression, assigned_expression), immediate_subpath) in unassigned_compound.as_slice().iter().zip(assigned_compound.as_slice().iter()).zip(immediate_subpaths) {
                    current_path.0.push(immediate_subpath);
                    Self::reverse_assignment_helper_inner(unassigned_subexpression, assigned_expression, assignments, current_path)?;
                    current_path.0.pop();
                }
                // If successful, return Ok
                Ok(())
            }, // If the unassigned value is a variable, insert into the assignments being constructed
            (UnassignedTblExpression::Variable(variable), expr) => {
                assignments.insert(*variable,expr.into()).map_err(|conflict|
                    TblReverseAssignmentError::VariableConflict(TblReverseAssignmentVariableConflictError(conflict))
                )
            },
            (UnassignedTblExpression::Compound(_), TblExpression::Atom(assigned_atom))
                => Err(TblReverseAssignmentError::compound_matched_with_atom(*assigned_atom, current_path.clone()))
        }
    }
}

trait TblPartialAssignmentHelper<UC: UnassignedTblExpressionCompound>: Default {
    fn get(&self, var: &TblExpressionVariable) -> Option<&UnassignedTblExpression<UC>>;
    fn insert(&mut self, var: TblExpressionVariable, expr: UnassignedTblExpression<UC>) -> Result<(),KeyConflictError<TblExpressionVariable,UnassignedTblExpression<UC>>>;

    
}

// impl <
//     T: TblAssignmentHelper<C>,
//     C: TblExpressionCompound + for<'a> From<&'a PostAssignmentCompound>,
//     PreAssignmentUcompound: UnassignedTblExpressionCompound,
//     PostAssignmentCompound: TblExpressionCompound + for<'a> From<&'a C> + FromIterator<TblExpression<PostAssignmentCompound>>
// > PropositionalAssignment<UnassignedTblExpression<PreAssignmentUcompound>,TblExpression<PostAssignmentCompound>>
// for T {
//     type AssignmentError = TblExpressionVariable;
//     type ReverseAssignmentError = TblReverseAssignmentError<C>;
//     fn assign(&self, subsuming_uprop: &UnassignedTblExpression<PreAssignmentUcompound>) -> Result<TblExpression<PostAssignmentCompound>,Self::AssignmentError> {
//         match subsuming_uprop {
//             UnassignedTblExpression::Atom(atom) => Ok(TblExpression::Atom(*atom)),
//             UnassignedTblExpression::Variable(variable) => match self.0.get(variable) {
//                 Some(expr) => Ok(expr.into()),
//                 None => Err(*variable),
//             }, UnassignedTblExpression::Compound(compound) => Ok(TblExpression::Compound(
//                 compound.get_immediate_subexpressions().into_iter()
//                     .map(|uexpr| self.assign(uexpr) )
//                     .try_collect()?
//             ))
//         }
//     }
// }
