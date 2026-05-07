use std::collections::HashMap;

use itertools::Itertools;
use proof_calculus::{propositions::assignments::{PartialPropositionalAssignmentConstructor, PropositionalAssignmentConstructor}, utils::{collections::maps::{KeyConflictError, dense_usize_map::DenseUsizeMap}, traits::{combinable::TryCombine, try_from_iter::TryFromIterator}}};

use crate::{expressions::{assignments::implementations::{dense::{DensePartialTblExpressionAssignment, DensePartialTblPropositionAssignment, DenseTblExpressionAssignment, DenseTblPropositionAssignment}, sparse::SparsePartialTblPropositionAssignment}, paths::TblSubexpressionInExpressionPath, types::{assigned::{TblExpression, compound::TblExpressionCompound, subexpressions::ParentOfSubexpressions}, unassigned::{compound::UnassignedTblExpressionCompound, subexpressions::ParentOfUnassignedSubexpressions, variable::TblExpressionVariable}}}, proof_calculus_derived::aliases::propositions::types::{TblProposition, UnassignedTblProposition}};

pub struct DenseTblExpressionAssignmentConstructor(DenseUsizeMap<TblExpressionVariable,TblSubexpressionInExpressionPath>);
pub type DenseTblPropositionAssignmentConstructor = DenseTblExpressionAssignmentConstructor;

impl Default for DenseTblExpressionAssignmentConstructor {
    fn default() -> Self { Self(Default::default()) }
}
impl TryFromIterator<(TblExpressionVariable,TblSubexpressionInExpressionPath)> for DenseTblExpressionAssignmentConstructor {
    type Error = KeyConflictError<TblExpressionVariable,TblSubexpressionInExpressionPath>;
    fn try_from_iter<T: IntoIterator<Item = (TblExpressionVariable,TblSubexpressionInExpressionPath)>>(iter: T) -> Result<Self,Self::Error>
        { Ok(Self(DenseUsizeMap::try_from_iter(iter.into_iter())?)) }
}
impl TryCombine for DenseTblExpressionAssignmentConstructor {
    type CombinationError = KeyConflictError<TblExpressionVariable,TblSubexpressionInExpressionPath>;
    fn combine<I: IntoIterator<Item = Self>>(assignments: I) -> Result<Self,Self::CombinationError>
        { Ok(Self(DenseUsizeMap::merge_without_conflicts(assignments.into_iter().map(|v| v.0))?)) }
        
}

impl <UC: UnassignedTblExpressionCompound, C: TblExpressionCompound + FromIterator<TblExpression<C>>>
PropositionalAssignmentConstructor<UnassignedTblProposition<UC>, TblProposition<C>, DenseTblPropositionAssignment<C>>
for DenseTblExpressionAssignmentConstructor {
    type Error = ();
    fn try_construct(&self, prop: &TblProposition<C>) -> Result<DenseTblPropositionAssignment<C>,()> {
        let inner =self.0
            .try_transform_values(|path| prop.get_subexpression(path).cloned())
            .map_err(|_|())?;
        Ok(DenseTblExpressionAssignment(inner))
    }
}



pub struct DensePartialTblExpressionAssignmentConstructor(DenseUsizeMap<TblExpressionVariable,TblSubexpressionInExpressionPath>);
pub type PartiaPartiallDenseTblPropositionAssignmentConstructor = DenseTblExpressionAssignmentConstructor;

impl Default for DensePartialTblExpressionAssignmentConstructor {
    fn default() -> Self { Self(Default::default()) }
}
impl TryFromIterator<(TblExpressionVariable,TblSubexpressionInExpressionPath)> for DensePartialTblExpressionAssignmentConstructor {
    type Error = KeyConflictError<TblExpressionVariable,TblSubexpressionInExpressionPath>;
    fn try_from_iter<T: IntoIterator<Item = (TblExpressionVariable,TblSubexpressionInExpressionPath)>>(iter: T) -> Result<Self,Self::Error>
        { Ok(Self(DenseUsizeMap::try_from_iter(iter.into_iter())?)) }
}
impl TryCombine for DensePartialTblExpressionAssignmentConstructor {
    type CombinationError = KeyConflictError<TblExpressionVariable,TblSubexpressionInExpressionPath>;
    fn combine<I: IntoIterator<Item = Self>>(assignments: I) -> Result<Self,Self::CombinationError>
        { Ok(Self(DenseUsizeMap::merge_without_conflicts(assignments.into_iter().map(|v| v.0))?)) }
}

impl <PreAssignmentUcompound: UnassignedTblExpressionCompound, PostAssignmentUcompound: for<'a> From<&'a PreAssignmentUcompound> + for<'a> From<&'a PostAssignmentUcompound> + UnassignedTblExpressionCompound>
PartialPropositionalAssignmentConstructor<UnassignedTblProposition<PreAssignmentUcompound>,UnassignedTblProposition<PostAssignmentUcompound>, DensePartialTblPropositionAssignment<PostAssignmentUcompound>>
for DenseTblExpressionAssignmentConstructor {
    type Error = ();
    fn try_construct(&self, prop: &UnassignedTblProposition<PostAssignmentUcompound>) -> Result<DensePartialTblPropositionAssignment<PostAssignmentUcompound>,()> {
        let inner =self.0
            .try_transform_values(|path| match prop.get_subexpression(path) {
                Ok(uexpr) => Ok(uexpr.into()),
                Err(err) => Err(err),
            }).map_err(|err| err.1)?;
        Ok(DensePartialTblExpressionAssignment(inner))
    }
}
impl <PreAssignmentUcompound: UnassignedTblExpressionCompound, PostAssignmentUcompound: for<'a> From<&'a PreAssignmentUcompound> + for<'a> From<&'a PostAssignmentUcompound> + UnassignedTblExpressionCompound>
PartialPropositionalAssignmentConstructor<UnassignedTblProposition<PreAssignmentUcompound>,UnassignedTblProposition<PostAssignmentUcompound>, SparsePartialTblPropositionAssignment<PostAssignmentUcompound>>
for DenseTblExpressionAssignmentConstructor {
    type Error = ();
    fn try_construct(&self, prop: &UnassignedTblProposition<PostAssignmentUcompound>) -> Result<SparsePartialTblPropositionAssignment<PostAssignmentUcompound>,()> {
        let values: HashMap<_,_> = self.0.iter()
            .map(|(variable,path)| Ok((variable,match prop.get_subexpression_owned(path) {
                Ok(uexpr) => uexpr.into(),
                Err(e) => return Err(e),
            })))
            .try_collect()?;
        Ok(SparsePartialTblPropositionAssignment::from(values))
    }
}
