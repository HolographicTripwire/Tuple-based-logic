use std::collections::HashMap;

use itertools::Itertools;
use proof_calculus::{propositions::assignments::{PartialPropositionalAssignmentConstructor, PropositionalAssignmentConstructor}, utils::{collections::maps::conflictless::{KeyConflictError, hashmap::ConflictlessHashMap}, traits::{combinable::TryCombine, try_from_iter::TryFromIterator}}};

use crate::{expressions::{assignments::implementations::{dense::DensePartialTblPropositionAssignment, sparse::{SparsePartialTblExpressionAssignment, SparsePartialTblPropositionAssignment, SparseTblExpressionAssignment, SparseTblPropositionAssignment}}, paths::TblSubexpressionInExpressionPath, types::{assigned::{TblExpression, compound::TblExpressionCompound, subexpressions::ParentOfSubexpressions}, unassigned::{UnassignedTblExpression, compound::UnassignedTblExpressionCompound, subexpressions::ParentOfUnassignedSubexpressions, variable::TblExpressionVariable}}}, proof_calculus_derived::aliases::propositions::types::{TblProposition, UnassignedTblProposition}};

#[derive(Clone,PartialEq,Eq,Debug)]
pub struct SparseTblExpressionAssignmentConstructor(pub ConflictlessHashMap<TblExpressionVariable,TblSubexpressionInExpressionPath>);
pub type SparseTblPropositionAssignmentConstructor = SparseTblExpressionAssignmentConstructor;

impl Default for SparseTblExpressionAssignmentConstructor { fn default() -> Self { Self(Default::default()) } }
impl From<HashMap<TblExpressionVariable,TblSubexpressionInExpressionPath>> for SparseTblExpressionAssignmentConstructor
    { fn from(map: HashMap<TblExpressionVariable,TblSubexpressionInExpressionPath>) -> Self { Self(ConflictlessHashMap::from(map)) } }
impl Into<HashMap<TblExpressionVariable,TblSubexpressionInExpressionPath>> for SparseTblExpressionAssignmentConstructor
    { fn into(self) -> HashMap<TblExpressionVariable,TblSubexpressionInExpressionPath> { self.0.into() } }
impl TryFromIterator<(TblExpressionVariable,TblSubexpressionInExpressionPath)> for SparseTblExpressionAssignmentConstructor {
    type Error = KeyConflictError<TblExpressionVariable,TblSubexpressionInExpressionPath>;
    fn try_from_iter<T: IntoIterator<Item = (TblExpressionVariable,TblSubexpressionInExpressionPath)>>(iter: T) -> Result<Self,Self::Error>
        { Ok(Self(ConflictlessHashMap::try_from_iter(iter.into_iter())?)) }
} impl TryCombine for SparseTblExpressionAssignmentConstructor {
    type CombinationError = KeyConflictError<TblExpressionVariable,TblSubexpressionInExpressionPath>;
    fn try_combine<I: IntoIterator<Item = Self>>(assignments: I) -> Result<Self,Self::CombinationError>
        { Ok(Self(ConflictlessHashMap::try_combine(assignments.into_iter().map(|v| v.0))?)) }
}

impl <
    C: TblExpressionCompound + for<'a> From<&'a PostAssignmentCompound>,
    PreAssignmentUcompound: UnassignedTblExpressionCompound,
    PostAssignmentCompound: TblExpressionCompound + for<'a> From<&'a C> + for<'a> From<&'a PreAssignmentUcompound> + FromIterator<TblExpression<PostAssignmentCompound>>
> PropositionalAssignmentConstructor<UnassignedTblProposition<PreAssignmentUcompound>, TblProposition<PostAssignmentCompound>, SparseTblPropositionAssignment<C>>
for SparseTblExpressionAssignmentConstructor {
    type Error = ();
    fn try_construct(&self, prop: &TblProposition<PostAssignmentCompound>) -> Result<SparseTblExpressionAssignment<C>, ()> {
        let inner: HashMap<_,_> =self.0.iter()
            .map(|(variable,path)| Ok((*variable,prop.get_subexpression(&path)?.into())))
            .try_collect()
            .map_err(|_: ()|())?;
        Ok(SparseTblExpressionAssignment(inner.into()))
    }
}

pub struct SparsePartialTblExpressionAssignmentConstructor(pub ConflictlessHashMap<TblExpressionVariable,TblSubexpressionInExpressionPath>);
pub type PartiaPartiallSparseTblPropositionAssignmentConstructor = SparseTblExpressionAssignmentConstructor;

impl Default for SparsePartialTblExpressionAssignmentConstructor { fn default() -> Self { Self(Default::default()) } }
impl From<HashMap<TblExpressionVariable,TblSubexpressionInExpressionPath>> for SparsePartialTblExpressionAssignmentConstructor
    { fn from(map: HashMap<TblExpressionVariable,TblSubexpressionInExpressionPath>) -> Self { Self(ConflictlessHashMap::from(map)) } }
impl Into<HashMap<TblExpressionVariable,TblSubexpressionInExpressionPath>> for SparsePartialTblExpressionAssignmentConstructor
    { fn into(self) -> HashMap<TblExpressionVariable,TblSubexpressionInExpressionPath> { self.0.into() } }
impl TryFromIterator<(TblExpressionVariable,TblSubexpressionInExpressionPath)> for SparsePartialTblExpressionAssignmentConstructor {
    type Error = KeyConflictError<TblExpressionVariable,TblSubexpressionInExpressionPath>;
    fn try_from_iter<T: IntoIterator<Item = (TblExpressionVariable,TblSubexpressionInExpressionPath)>>(iter: T) -> Result<Self,Self::Error>
        { Ok(Self(ConflictlessHashMap::try_from_iter(iter.into_iter())?)) }
} impl TryCombine for SparsePartialTblExpressionAssignmentConstructor {
    type CombinationError = KeyConflictError<TblExpressionVariable,TblSubexpressionInExpressionPath>;
    fn try_combine<I: IntoIterator<Item = Self>>(assignments: I) -> Result<Self,Self::CombinationError>
        { Ok(Self(ConflictlessHashMap::try_combine(assignments.into_iter().map(|v| v.0))?)) }
}

impl <
    PreAssignmentUcompound: UnassignedTblExpressionCompound,
    PostAssignmentUcompound: UnassignedTblExpressionCompound + for<'a> From<&'a PreAssignmentUcompound> + for<'a> From<&'a PostAssignmentUcompound> + FromIterator<UnassignedTblExpression<PostAssignmentUcompound>>
> PartialPropositionalAssignmentConstructor<UnassignedTblProposition<PreAssignmentUcompound>,UnassignedTblProposition<PostAssignmentUcompound>, SparsePartialTblPropositionAssignment<PostAssignmentUcompound>>
for SparseTblExpressionAssignmentConstructor {
    type Error = ();
    fn try_construct(&self, prop: &UnassignedTblExpression<PostAssignmentUcompound>) -> Result<SparsePartialTblExpressionAssignment<PostAssignmentUcompound>, ()> {
        let inner: HashMap<_,_> =self.0.iter()
            .map(|(variable,path)| Ok((*variable,match prop.get_subexpression(&path) {
                Ok(uexpr) => uexpr.into(),
                Err(err) => return Err(err),
            })))
            .try_collect()
            .map_err(|_: ()|())?;
        Ok(SparsePartialTblExpressionAssignment(inner.into()))
    }
}
impl <
    PreAssignmentUcompound: UnassignedTblExpressionCompound,
    PostAssignmentUcompound: UnassignedTblExpressionCompound + for<'a> From<&'a PreAssignmentUcompound> + for<'a> From<&'a PostAssignmentUcompound> + FromIterator<UnassignedTblExpression<PostAssignmentUcompound>>
> PartialPropositionalAssignmentConstructor<UnassignedTblProposition<PreAssignmentUcompound>,UnassignedTblProposition<PostAssignmentUcompound>, DensePartialTblPropositionAssignment<PostAssignmentUcompound>>
for SparseTblExpressionAssignmentConstructor {
    type Error = ();
    fn try_construct(&self, prop: &UnassignedTblProposition<PostAssignmentUcompound>) -> Result<DensePartialTblPropositionAssignment<PostAssignmentUcompound>,()> {
        let values: Vec<_>= self.0.iter()
            .map(|(variable,path)| Ok((*variable,match prop.get_subexpression(path) {
                Ok(uexpr) => uexpr.into(),
                Err(err) => return Err(err),
            })))
            .try_collect()?;
        Ok(DensePartialTblPropositionAssignment::from_iter_unchecked(values))
    }
}
