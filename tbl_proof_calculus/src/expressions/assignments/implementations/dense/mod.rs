use proof_calculus::utils::{collections::maps::{KeyConflictError, dense_usize_map::DenseUsizeMap}, traits::{combinable::{TryCombine}, try_from_iter::TryFromIterator}};

use crate::expressions::{types::{assigned::{TblExpression, compound::TblExpressionCompound}, unassigned::{UnassignedTblExpression, compound::UnassignedTblExpressionCompound, variable::TblExpressionVariable}}};

pub mod constructors;

#[derive(Clone,PartialEq,Eq,Debug)]
pub struct DenseTblExpressionAssignment<C: TblExpressionCompound>(DenseUsizeMap<TblExpressionVariable,TblExpression<C>>);
pub type DenseTblPropositionAssignment<C: TblExpressionCompound> = DenseTblExpressionAssignment<C>;
impl <C: TblExpressionCompound> DenseTblExpressionAssignment<C> {
    pub fn from_iter_unchecked<T: IntoIterator<Item = (TblExpressionVariable,TblExpression<C>)>>(iter: T) -> Self 
        { Self(DenseUsizeMap::from_iter_unchecked(iter)) }
}
impl <C: TblExpressionCompound> Default for DenseTblExpressionAssignment<C> {
    fn default() -> Self { Self(Default::default()) }
}
impl <C: TblExpressionCompound> TryFromIterator<(TblExpressionVariable,TblExpression<C>)> for DenseTblExpressionAssignment<C> {
    type Error = KeyConflictError<TblExpressionVariable,TblExpression<C>>;
    fn try_from_iter<T: IntoIterator<Item = (TblExpressionVariable,TblExpression<C>)>>(iter: T) -> Result<Self,Self::Error>
        { Ok(Self(DenseUsizeMap::try_from_iter(iter.into_iter())?)) }
}
impl <C: TblExpressionCompound> TryCombine for DenseTblExpressionAssignment<C> {
    type CombinationError = KeyConflictError<TblExpressionVariable,TblExpression<C>>;
    fn combine<I: IntoIterator<Item = Self>>(assignments: I) -> Result<Self,Self::CombinationError>
        { Ok(Self(DenseUsizeMap::merge_without_conflicts(assignments.into_iter().map(|v| v.0))?)) }
}
// impl <C: TblExpressionCompound + FromIterator<TblExpression<C>>, UC: UnassignedTblExpressionCompound>
// PropositionalAssignment<UnassignedTblExpression<UC>,TblExpression<C>>
// for DenseTblExpressionAssignment<C> {
//     type AssignmentError = ();
//     type ReverseAssignmentError = ();
//     fn assign(&self, uprop: &UnassignedTblExpression<UC>) -> Result<TblExpression<C>,Self::AssignmentError> {
//         match uprop {
//             UnassignedTblExpression::Atom(atom) => Ok(TblExpression::Atom(*atom)),
//             UnassignedTblExpression::Variable(variable) => match self.0.get(variable) {
//                 Some(expr) => Ok(expr.clone()),
//                 None => Err(()),
//             }, UnassignedTblExpression::Compound(compound) => Ok(TblExpression::Compound(
//                 compound.get_immediate_subexpressions().into_iter()
//                     .map(|uexpr| self.assign(uexpr) )
//                     .try_collect()?
//             ))
//         }
//     }
//     fn reverse_assign(from: &UnassignedTblExpression<UC>, to: &TblExpression<C>) -> Result<Self,Self::ReverseAssignmentError> {
//         todo!()
//     }
// }

#[derive(Clone,PartialEq,Eq,Debug)]
pub struct DensePartialTblExpressionAssignment<UC: UnassignedTblExpressionCompound>(DenseUsizeMap<TblExpressionVariable,UnassignedTblExpression<UC>>);
pub type DensePartialTblPropositionAssignment<UC: TblExpressionCompound> = DensePartialTblExpressionAssignment<UC>;
impl <UC: UnassignedTblExpressionCompound> DensePartialTblExpressionAssignment<UC> {
    pub fn from_iter_unchecked<T: IntoIterator<Item = (TblExpressionVariable,UnassignedTblExpression<UC>)>>(iter: T) -> Self 
        { Self(DenseUsizeMap::from_iter_unchecked(iter)) }
}
impl <UC: UnassignedTblExpressionCompound> Default for DensePartialTblExpressionAssignment<UC> {
    fn default() -> Self { Self(Default::default()) }
}
impl <UC: UnassignedTblExpressionCompound> TryFromIterator<(TblExpressionVariable,UnassignedTblExpression<UC>)> for DensePartialTblExpressionAssignment<UC> {
    type Error = KeyConflictError<TblExpressionVariable,UnassignedTblExpression<UC>>;
    fn try_from_iter<T: IntoIterator<Item = (TblExpressionVariable,UnassignedTblExpression<UC>)>>(iter: T) -> Result<Self,Self::Error>
        { Ok(Self(DenseUsizeMap::try_from_iter(iter.into_iter())?)) }
}
impl <UC: UnassignedTblExpressionCompound> TryCombine for DensePartialTblExpressionAssignment<UC> {
    type CombinationError = KeyConflictError<TblExpressionVariable,UnassignedTblExpression<UC>>;
    fn combine<I: IntoIterator<Item = Self>>(assignments: I) -> Result<Self,Self::CombinationError>
        { Ok(Self(DenseUsizeMap::merge_without_conflicts(assignments.into_iter().map(|v| v.0))?)) }
} 
// impl <
//     FromUcompound: UnassignedTblExpressionCompound,
//     ToUcompound: for<'a> From<&'a SelfUcompound> + for<'a> From<&'a FromUcompound> + UnassignedTblExpressionCompound,
//     SelfUcompound: UnassignedTblExpressionCompound
// > PartialPropositionalAssignment<UnassignedTblExpression<FromUcompound>,UnassignedTblExpression<ToUcompound>>
// for DensePartialTblExpressionAssignment<SelfUcompound> {
//     type AssignmentError = Infallible;
//     type ReverseAssignmentError = ();
//     fn assign(&self, subsuming_uprop: &UnassignedTblExpression<FromUcompound>) -> Result<UnassignedTblExpression<ToUcompound>,Self::AssignmentError> {
//         Ok(match subsuming_uprop {
//             UnassignedTblExpression::Variable(variable) => match self.0.get(variable) {
//                 Some(uexpr) => uexpr.into(),
//                 None => UnassignedTblExpression::Variable(*variable),
//             }, other => other.into(),
//         })
//     }
//     fn reverse_assign(subsuming_uprop: &UnassignedTblExpression<FromUcompound>, subsumed_uprop: &UnassignedTblExpression<ToUcompound>) -> Result<Self,Self::ReverseAssignmentError> {
//         todo!()
//     }
// }
// TODO: consider performance implications of having a From<Vec> implementation for these to leverage From<Vec> of underlying DenseUsizeMap
