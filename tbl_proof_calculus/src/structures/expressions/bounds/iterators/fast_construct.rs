use std::collections::{HashSet};

use proof_calculus::structures::propositions::bounds::PropositionIdentityBounds;

use crate::structures::expressions::{TblExpression, at_path_enum::TblSubexpressionInExpressionEnum, bounds::{TblExpressionBoundAtomExactValue, TblExpressionBoundCompoundExactLength, TblExpressionIdentityBound, iterators::ImportantPathsConstructor}, compound::CompoundTblExpression, subexpressions::{TblSubexpressionInExpression, TblSubexpressionInExpressionPath, immediate::LocatedParentOfImmediateSubexpressions}};

/// [PropositionIdentityBounds] for [TblProposition] which is fast to construct
/// To see [PropositionIdentityBounds] for [TblProposition] which fast to perform lookups with, see [TblExpressionFastLookupIdentityBounds]
pub struct TblExpressionFastConstructIdentityBounds<'a, C: CompoundTblExpression>(&'a TblExpression<C>);
pub type TblPropositionFastConstructIdentityBounds<'a, C: CompoundTblExpression> = TblExpressionFastConstructIdentityBounds<'a,C>;

impl <'a, C: CompoundTblExpression> PropositionIdentityBounds<'a, TblExpression<C>,TblExpressionIdentityBound> for TblExpressionFastConstructIdentityBounds<'a,C> {
    fn new(expr: &'a TblExpression<C>) -> Self { Self(expr) }
}
impl <'a, C: CompoundTblExpression> TblExpressionFastConstructIdentityBounds<'a,C> {
    pub (super) fn bounds_and_important(self) -> (Vec<TblExpressionIdentityBound>,HashSet<TblSubexpressionInExpressionPath>) {
        let mut bounds = Vec::new();
        let mut important = ImportantPathsConstructor::default();
        for bound in self.into_iter() {
            important.try_insert(bound.path().clone());
            bounds.push(bound);
        }
        (bounds,important.construct())
    }
}
impl <'a, C: CompoundTblExpression> IntoIterator for TblExpressionFastConstructIdentityBounds<'a,C> {
    type Item = TblExpressionIdentityBound;
    type IntoIter = TblExpressionFastConstructIdentityBoundsIterator<'a,C>;

    fn into_iter(self) -> Self::IntoIter { TblExpressionFastConstructIdentityBoundsIterator::new(self.0) }
}

pub (super) struct TblExpressionFastConstructIdentityBoundsIterator<'a,C: CompoundTblExpression> {
    exprs: Vec<TblSubexpressionInExpression<'a, C>>
}
impl <'a, C: CompoundTblExpression> TblExpressionFastConstructIdentityBoundsIterator<'a,C> {
    pub fn new(expr: &'a TblExpression<C>) -> Self {
        Self { exprs: vec![TblSubexpressionInExpression { obj: expr, path: TblSubexpressionInExpressionPath::default() }] }
    }

    fn process(&mut self, expr: TblSubexpressionInExpression<'a,C>) -> TblExpressionIdentityBound {
        match expr.into() {
            TblSubexpressionInExpressionEnum::Atomic(atom) =>
                { TblExpressionBoundAtomExactValue::new(atom.path, *atom.obj).into() },
            TblSubexpressionInExpressionEnum::Compound(compound) => {
                let (path, length) = (compound.path.clone(), compound.obj.len());
                self.exprs.extend(compound.into_located_immediate_subexpressions());
                TblExpressionBoundCompoundExactLength::new(path, length).into()
            }
        }
    }

}
impl <'a, C: CompoundTblExpression> Iterator for TblExpressionFastConstructIdentityBoundsIterator<'a,C> {
    type Item = TblExpressionIdentityBound;
    fn next(&mut self) -> Option<Self::Item>
        { self.exprs.pop().map(|expr| self.process(expr)) }
}
