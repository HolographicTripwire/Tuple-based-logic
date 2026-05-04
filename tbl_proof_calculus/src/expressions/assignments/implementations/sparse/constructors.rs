use std::collections::HashMap;

use itertools::Itertools;
use proof_calculus::{propositions::assignments::{PartialPropositionalAssignmentConstructor, PropositionalAssignmentConstructor}, utils::{collections::maps::{KeyConflictError, conflictless_hashmap::{ConflictlessHashMap}}, traits::{combinable::TryCombine, try_from_iter::TryFromIterator}}};

use crate::{expressions::{assignments::implementations::{dense::DensePartialTblPropositionAssignment, sparse::{SparsePartialTblExpressionAssignment, SparsePartialTblPropositionAssignment, SparseTblExpressionAssignment, SparseTblPropositionAssignment}}, paths::TblSubexpressionInExpressionPath, types::{assigned::{TblExpression, compound::CompoundTblExpression, subexpressions::ParentOfSubexpressions}, unassigned::{compound::UnassignedCompoundTblExpression, subexpressions::ParentOfUnassignedSubexpressions, variable::TblExpressionVariable}}}, proof_calculus_derived::aliases::propositions::types::{TblProposition, UnassignedTblProposition}};

#[derive(Clone,PartialEq,Eq,Debug,Default)]
pub struct SparseTblExpressionAssignmentConstructor(pub ConflictlessHashMap<TblExpressionVariable,TblSubexpressionInExpressionPath>);
pub type SparseTblPropositionAssignmentConstructor = SparseTblExpressionAssignmentConstructor;

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
    fn combine<I: IntoIterator<Item = Self>>(assignments: I) -> Result<Self,Self::CombinationError>
        { Ok(Self(ConflictlessHashMap::combine(assignments.into_iter().map(|v| v.0))?)) }
}

impl <UC: UnassignedCompoundTblExpression, C: CompoundTblExpression + FromIterator<TblExpression<C>>> PropositionalAssignmentConstructor<UnassignedTblProposition<UC>, TblProposition<C>, SparseTblPropositionAssignment<C>> for SparseTblExpressionAssignmentConstructor {
    type Error = ();
    fn try_construct(&self, prop: &TblProposition<C>) -> Result<SparseTblPropositionAssignment<C>,()> {
        let inner: HashMap<_,_> =self.0.iter()
            .map(|(variable,path)| Ok((*variable,prop.get_subexpression_owned(&path)?.clone())))
            .try_collect()
            .map_err(|_: ()|())?;
        Ok(SparseTblExpressionAssignment(inner.into()))
    }
}

pub struct SparsePartialTblExpressionAssignmentConstructor(pub ConflictlessHashMap<TblExpressionVariable,TblSubexpressionInExpressionPath>);
pub type PartiaPartiallSparseTblPropositionAssignmentConstructor = SparseTblExpressionAssignmentConstructor;

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
    fn combine<I: IntoIterator<Item = Self>>(assignments: I) -> Result<Self,Self::CombinationError>
        { Ok(Self(ConflictlessHashMap::combine(assignments.into_iter().map(|v| v.0))?)) }
}

impl <'assignment,'from: 'from2, 'from2, FromUcompound: 'from + UnassignedCompoundTblExpression, ToUcompound: 'assignment + for<'a> From<&'a FromUcompound> + From<&'assignment ToUcompound> + UnassignedCompoundTblExpression>
PartialPropositionalAssignmentConstructor<'assignment,'from2,UnassignedTblProposition<FromUcompound>,UnassignedTblProposition<ToUcompound>, SparsePartialTblPropositionAssignment<ToUcompound>>
for SparseTblExpressionAssignmentConstructor {
    type Error = ();
    fn try_construct(&self, prop: &UnassignedTblProposition<FromUcompound>) -> Result<SparsePartialTblPropositionAssignment<ToUcompound>,()> {
        let inner: HashMap<_,_> =self.0.iter()
            .map(|(variable,path)| Ok((*variable,match prop.get_subexpression_owned(&path) {
                Ok(uexpr) => uexpr.transmute_compound(),
                Err(err) => return Err(err),
            })))
            .try_collect()
            .map_err(|_: ()|())?;
        Ok(SparsePartialTblExpressionAssignment(inner.into()))
    }
}
impl <'assignment,'from: 'from2, 'from2, FromUcompound: 'from + UnassignedCompoundTblExpression, ToUcompound: 'assignment + for<'a> From<&'a FromUcompound> + From<&'assignment ToUcompound> + UnassignedCompoundTblExpression>
PartialPropositionalAssignmentConstructor<'assignment,'from2,UnassignedTblProposition<FromUcompound>,UnassignedTblProposition<ToUcompound>, DensePartialTblPropositionAssignment<ToUcompound>>
for SparseTblExpressionAssignmentConstructor {
    type Error = ();
    fn try_construct(&self, prop: &UnassignedTblProposition<FromUcompound>) -> Result<DensePartialTblPropositionAssignment<ToUcompound>,()> {
        let values: Vec<_>= self.0.iter()
            .map(|(variable,path)| Ok((*variable,match prop.get_subexpression_owned(path) {
                Ok(uexpr) => uexpr.transmute_compound(),
                Err(err) => return Err(err),
            })))
            .try_collect()?;
        Ok(DensePartialTblPropositionAssignment::from_iter_unchecked(values))
    }
}