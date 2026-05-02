use std::collections::HashMap;

use itertools::Itertools;
use proof_calculus::{propositions::assignments::{PartialPropositionalAssignmentConstructor, PropositionalAssignmentConstructor}, utils::{collections::maps::{KeyConflictError, hashmap::{combine_hashmaps_without_conflicts, create_hashmap_without_conflicts}}, traits::try_from_iter::TryFromIterator}};

use crate::{expressions::{assignments::implementations::{dense::DensePartialTblPropositionAssignment, sparse::{SparsePartialTblExpressionAssignment, SparsePartialTblPropositionAssignment, SparseTblExpressionAssignment, SparseTblPropositionAssignment}}, paths::TblSubexpressionInExpressionPath, types::{assigned::{TblExpression, compound::CompoundTblExpression, subexpressions::ParentOfSubexpressions}, unassigned::{compound::UnassignedCompoundTblExpression, subexpressions::ParentOfUnassignedSubexpressions, variable::TblExpressionVariable}}}, proof_calculus_derived::aliases::propositions::types::{TblProposition, UnassignedTblProposition}};

pub struct SparseTblExpressionAssignmentConstructor(HashMap<TblExpressionVariable,TblSubexpressionInExpressionPath>);
pub type SparseTblPropositionAssignmentConstructor = SparseTblExpressionAssignmentConstructor;

impl Default for SparseTblExpressionAssignmentConstructor {
    fn default() -> Self { Self(Default::default()) }
}
impl TryFromIterator<(TblExpressionVariable,TblSubexpressionInExpressionPath)> for SparseTblExpressionAssignmentConstructor {
    type Error = KeyConflictError<TblExpressionVariable,TblSubexpressionInExpressionPath>;
    fn try_from_iter<T: IntoIterator<Item = (TblExpressionVariable,TblSubexpressionInExpressionPath)>>(iter: T) -> Result<Self,Self::Error>
        { Ok(Self(create_hashmap_without_conflicts(iter)?)) }
}
impl SparseTblExpressionAssignmentConstructor {
    fn from_iter_unchecked<T: IntoIterator<Item = (TblExpressionVariable,TblSubexpressionInExpressionPath)>>(iter: T) -> Self 
        { Self(HashMap::from_iter(iter)) }
    fn combine<I: IntoIterator<Item = Self>>(assignments: I) -> Result<Self,KeyConflictError<TblExpressionVariable,TblSubexpressionInExpressionPath>>
        { Ok(Self(combine_hashmaps_without_conflicts(assignments.into_iter().map(|v| v.0))?)) }
}

impl <UC: UnassignedCompoundTblExpression, C: CompoundTblExpression + FromIterator<TblExpression<C>>> PropositionalAssignmentConstructor<UnassignedTblProposition<UC>, TblProposition<C>, SparseTblPropositionAssignment<C>> for SparseTblExpressionAssignmentConstructor {
    type Error = ();
    fn try_construct(&self, prop: &TblProposition<C>) -> Result<SparseTblPropositionAssignment<C>,()> {
        let inner =self.0.iter()
            .map(|(variable,path)| Ok((*variable,prop.get_subexpression_owned(&path)?.clone())))
            .try_collect()
            .map_err(|_: ()|())?;
        Ok(SparseTblExpressionAssignment(inner))
    }
}

pub struct SparsePartialTblExpressionAssignmentConstructor(HashMap<TblExpressionVariable,TblSubexpressionInExpressionPath>);
pub type PartiaPartiallSparseTblPropositionAssignmentConstructor = SparseTblExpressionAssignmentConstructor;

impl Default for SparsePartialTblExpressionAssignmentConstructor {
    fn default() -> Self { Self(Default::default()) }
}
impl TryFromIterator<(TblExpressionVariable,TblSubexpressionInExpressionPath)> for SparsePartialTblExpressionAssignmentConstructor {
    type Error = KeyConflictError<TblExpressionVariable,TblSubexpressionInExpressionPath>;
    fn try_from_iter<T: IntoIterator<Item = (TblExpressionVariable,TblSubexpressionInExpressionPath)>>(iter: T) -> Result<Self,Self::Error>
        { Ok(Self(create_hashmap_without_conflicts(iter)?)) }
}
impl SparsePartialTblExpressionAssignmentConstructor {
    fn from_iter_unchecked<T: IntoIterator<Item = (TblExpressionVariable,TblSubexpressionInExpressionPath)>>(iter: T) -> Self 
        { Self(HashMap::from_iter(iter)) }
    fn combine<I: IntoIterator<Item = Self>>(assignments: I) -> Result<Self,KeyConflictError<TblExpressionVariable,TblSubexpressionInExpressionPath>>
        { Ok(Self(combine_hashmaps_without_conflicts(assignments.into_iter().map(|v| v.0))?)) }
}

impl <'assignment,'from: 'from2, 'from2, FromUcompound: 'from + UnassignedCompoundTblExpression, ToUcompound: 'assignment + for<'a> From<&'a FromUcompound> + From<&'assignment ToUcompound> + UnassignedCompoundTblExpression>
PartialPropositionalAssignmentConstructor<'assignment,'from2,UnassignedTblProposition<FromUcompound>,UnassignedTblProposition<ToUcompound>, SparsePartialTblPropositionAssignment<ToUcompound>>
for SparseTblExpressionAssignmentConstructor {
    type Error = ();
    fn try_construct(&self, prop: &UnassignedTblProposition<FromUcompound>) -> Result<SparsePartialTblPropositionAssignment<ToUcompound>,()> {
        let inner =self.0.iter()
            .map(|(variable,path)| Ok((*variable,match prop.get_subexpression_owned(&path) {
                Ok(uexpr) => uexpr.transmute_compound(),
                Err(err) => return Err(err),
            })))
            .try_collect()
            .map_err(|_: ()|())?;
        Ok(SparsePartialTblExpressionAssignment(inner))
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