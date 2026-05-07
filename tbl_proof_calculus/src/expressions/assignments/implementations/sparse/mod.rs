use std::{collections::HashMap, convert::Infallible};

use proof_calculus::{propositions::assignments::{PartialPropositionalAssignment, PropositionalAssignment}, utils::{collections::maps::{KeyConflictError, conflictless_hashmap::ConflictlessHashMap}, traits::{combinable::TryCombine, try_from_iter::TryFromIterator}}};

use crate::expressions::{assignments::{errors::{assignment::TblAssignmentError, reverse_assignment::TblReverseAssignmentError}, implementations::TblAssignmentHelper}, types::{assigned::{TblExpression, compound::TblExpressionCompound}, unassigned::{UnassignedTblExpression, compound::UnassignedTblExpressionCompound, variable::TblExpressionVariable}}};

pub mod constructors;

#[derive(Clone,PartialEq,Eq,Debug)]
pub struct SparseTblExpressionAssignment<C: TblExpressionCompound>(pub ConflictlessHashMap<TblExpressionVariable, TblExpression<C>>);
pub type SparseTblPropositionAssignment<C: TblExpressionCompound> = SparseTblExpressionAssignment<C>;

impl <C: TblExpressionCompound> Default for SparseTblExpressionAssignment<C>
    { fn default() -> Self { Self(Default::default()) } }
impl <C: TblExpressionCompound> From<HashMap<TblExpressionVariable,TblExpression<C>>>
for SparseTblExpressionAssignment<C>
    { fn from(map: HashMap<TblExpressionVariable,TblExpression<C>>) -> Self { Self(ConflictlessHashMap::from(map)) } }
impl <C: TblExpressionCompound> Into<HashMap<TblExpressionVariable,TblExpression<C>>>
for SparseTblExpressionAssignment<C>
    { fn into(self) -> HashMap<TblExpressionVariable,TblExpression<C>> { self.0.into() } }
impl <C: TblExpressionCompound> TryFromIterator<(TblExpressionVariable,TblExpression<C>)>
for SparseTblExpressionAssignment<C> {
    type Error = KeyConflictError<TblExpressionVariable,TblExpression<C>>;
    fn try_from_iter<T: IntoIterator<Item = (TblExpressionVariable,TblExpression<C>)>>(iter: T) -> Result<Self,Self::Error>
        { Ok(Self(ConflictlessHashMap::try_from_iter(iter.into_iter())?)) }
} impl <C: TblExpressionCompound> TryCombine for SparseTblExpressionAssignment<C> {
    type CombinationError = KeyConflictError<TblExpressionVariable,TblExpression<C>>;
    fn combine<I: IntoIterator<Item = Self>>(assignments: I) -> Result<Self,Self::CombinationError>
        { Ok(Self(ConflictlessHashMap::combine(assignments.into_iter().map(|v| v.0))?)) }
}

// impl <
//     C: TblExpressionCompound + for<'a> From<&'a PostAssignmentCompound>,
//     PreAssignmentUcompound: UnassignedTblExpressionCompound,
//     PostAssignmentCompound: TblExpressionCompound + for<'a> From<&'a C> + FromIterator<TblExpression<PostAssignmentCompound>>
// > PropositionalAssignment<UnassignedTblExpression<PreAssignmentUcompound>,TblExpression<PostAssignmentCompound>>
// for SparseTblExpressionAssignment<C> {
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
//     fn reverse_assign(pre_assignment_uprop: &UnassignedTblExpression<PreAssignmentUcompound>, post_assignment_prop: &TblExpression<PostAssignmentCompound>) -> Result<Self,Self::ReverseAssignmentError> {
//         let mut assignments = Self::default();
//         let mut path = TblSubexpressionInExpressionPath::default();
//         Self::reverse_assign_helper(pre_assignment_uprop, post_assignment_prop, &mut assignments, &mut path)
//             .map(|_| assignments)
//     }
// }

impl <C: TblExpressionCompound> TblAssignmentHelper<C> for SparseTblExpressionAssignment<C> {
    fn get(&self, var: &TblExpressionVariable) -> Option<&TblExpression<C>>
        { self.0.get(var) }
    fn insert(&mut self, var: TblExpressionVariable, expr: TblExpression<C>) -> Result<(),KeyConflictError<TblExpressionVariable,TblExpression<C>>>
        { self.0.insert(var, expr) }
}
impl <
    C: TblExpressionCompound + for<'a> From<&'a PostAssignmentCompound>,
    PreAssignmentUcompound: UnassignedTblExpressionCompound,
    PostAssignmentCompound: TblExpressionCompound + for<'a> From<&'a C> + for<'a> From<&'a PreAssignmentUcompound> + FromIterator<TblExpression<PostAssignmentCompound>>
> PropositionalAssignment<UnassignedTblExpression<PreAssignmentUcompound>,TblExpression<PostAssignmentCompound>>
for SparseTblExpressionAssignment<C> {
    type AssignmentError = TblAssignmentError;
    type ReverseAssignmentError = TblReverseAssignmentError<C>;
    fn assign(&self, pre_assignment_uprop: &UnassignedTblExpression<PreAssignmentUcompound>) -> Result<TblExpression<PostAssignmentCompound>,Self::AssignmentError>
        { self.assign_helper::<PreAssignmentUcompound,PostAssignmentCompound>(pre_assignment_uprop) }
    fn reverse_assign(pre_assignment_uprop: &UnassignedTblExpression<PreAssignmentUcompound>, post_assignment_prop: &TblExpression<PostAssignmentCompound>) -> Result<Self,Self::ReverseAssignmentError>
        { Self::reverse_assign_helper::<PreAssignmentUcompound,PostAssignmentCompound>(pre_assignment_uprop, post_assignment_prop) }
}

#[derive(Clone,PartialEq,Eq,Debug)]
pub struct SparsePartialTblExpressionAssignment<PostAssignmentUcompound: UnassignedTblExpressionCompound>(pub ConflictlessHashMap<TblExpressionVariable, UnassignedTblExpression<PostAssignmentUcompound>>);
pub type SparsePartialTblPropositionAssignment<PostAssignmentUcompound: TblExpressionCompound> = SparsePartialTblExpressionAssignment<PostAssignmentUcompound>;

impl <PostAssignmentUcompound: UnassignedTblExpressionCompound> Default for SparsePartialTblExpressionAssignment<PostAssignmentUcompound>
    { fn default() -> Self { Self(Default::default()) } }
impl <PostAssignmentUcompound: UnassignedTblExpressionCompound> From<HashMap<TblExpressionVariable,UnassignedTblExpression<PostAssignmentUcompound>>>
for SparsePartialTblExpressionAssignment<PostAssignmentUcompound>
    { fn from(map: HashMap<TblExpressionVariable,UnassignedTblExpression<PostAssignmentUcompound>>) -> Self { Self(ConflictlessHashMap::from(map)) } }
impl <PostAssignmentUcompound: UnassignedTblExpressionCompound> Into<HashMap<TblExpressionVariable,UnassignedTblExpression<PostAssignmentUcompound>>>
for SparsePartialTblExpressionAssignment<PostAssignmentUcompound>
    { fn into(self) -> HashMap<TblExpressionVariable,UnassignedTblExpression<PostAssignmentUcompound>> { self.0.into() } }
impl <PostAssignmentUcompound: UnassignedTblExpressionCompound> TryFromIterator<(TblExpressionVariable,UnassignedTblExpression<PostAssignmentUcompound>)>
for SparsePartialTblExpressionAssignment<PostAssignmentUcompound> {
    type Error = KeyConflictError<TblExpressionVariable,UnassignedTblExpression<PostAssignmentUcompound>>;
    fn try_from_iter<T: IntoIterator<Item = (TblExpressionVariable,UnassignedTblExpression<PostAssignmentUcompound>)>>(iter: T) -> Result<Self,Self::Error>
        { Ok(Self(ConflictlessHashMap::try_from_iter(iter.into_iter())?)) }
} impl <PostAssignmentUcompound: UnassignedTblExpressionCompound> TryCombine for SparsePartialTblExpressionAssignment<PostAssignmentUcompound> {
    type CombinationError = KeyConflictError<TblExpressionVariable,UnassignedTblExpression<PostAssignmentUcompound>>;
    fn combine<I: IntoIterator<Item = Self>>(assignments: I) -> Result<Self,Self::CombinationError>
        { Ok(Self(ConflictlessHashMap::combine(assignments.into_iter().map(|v| v.0))?)) }
}

impl <
    SelfUcompound: UnassignedTblExpressionCompound,
    PreAssignmentUcompound: UnassignedTblExpressionCompound,
    PostAssignmentUcompound: for<'a> From<&'a SelfUcompound> + for<'a> From<&'a PreAssignmentUcompound> + UnassignedTblExpressionCompound
> PartialPropositionalAssignment<UnassignedTblExpression<PreAssignmentUcompound>,UnassignedTblExpression<PostAssignmentUcompound>>
for SparsePartialTblExpressionAssignment<SelfUcompound> {
    type AssignmentError = Infallible;
    type ReverseAssignmentError = ();
    fn assign(&self, subsuming_uprop: &UnassignedTblExpression<PreAssignmentUcompound>) -> Result<UnassignedTblExpression<PostAssignmentUcompound>,Self::AssignmentError> {
        Ok(match subsuming_uprop {
            UnassignedTblExpression::Variable(variable) => match self.0.get(variable) {
                Some(uexpr) => uexpr.into(),
                None => UnassignedTblExpression::Variable(*variable),
            }, other => other.into(),
        })
    }
    fn reverse_assign(from: &UnassignedTblExpression<PreAssignmentUcompound>, to: &UnassignedTblExpression<PostAssignmentUcompound>) -> Result<Self,Self::ReverseAssignmentError> {
        todo!()
    }
}
