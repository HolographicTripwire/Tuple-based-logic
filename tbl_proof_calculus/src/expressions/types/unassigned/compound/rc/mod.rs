use std::rc::Rc;

use crate::expressions::{paths::{TblSubexpressionInExpressionPath, immediate::ImmediateTblSubexpressionInExpressionPath}, types::unassigned::{UnassignedTblExpression, compound::UnassignedTblExpressionCompound, subexpressions::{ParentOfUnassignedSubexpressions, immediate::ParentOfImmediateUnassignedSubexpressions}}};

/// A compound unit in Tuple-Based Logic, which are used to build up [Propositions](Proposition)
#[derive(Debug,Clone,PartialEq,Eq,Hash)]
pub struct UnassignedRcCompoundTblExpression(pub Rc<[UnassignedTblExpression<UnassignedRcCompoundTblExpression>]>);
impl UnassignedTblExpressionCompound for UnassignedRcCompoundTblExpression {
    fn len(&self) -> usize { self.0.len() }
    fn as_slice(&self) -> &[UnassignedTblExpression<Self>] { &self.0 }
    fn replace(&self, to_replace: &UnassignedTblExpression<Self>, replace_with: &UnassignedTblExpression<Self>) -> Self
        { self.0.iter().map(|v| v.replace(to_replace, replace_with)).collect() }
}

impl ParentOfImmediateUnassignedSubexpressions<UnassignedRcCompoundTblExpression> for UnassignedRcCompoundTblExpression {
    fn get_immediate_subexpression_paths(&self) -> impl IntoIterator<Item = ImmediateTblSubexpressionInExpressionPath>
        { (0..self.0.len()).map(|x| x.into()) }
    fn get_immediate_subexpression(&self,path: &ImmediateTblSubexpressionInExpressionPath) -> Result<&UnassignedTblExpression<UnassignedRcCompoundTblExpression>, ()>
        { self.0.get(path.0).ok_or(()) }
}

impl ParentOfUnassignedSubexpressions<UnassignedRcCompoundTblExpression> for UnassignedRcCompoundTblExpression {
    fn get_subexpression_paths(&self) -> impl IntoIterator<Item = TblSubexpressionInExpressionPath>  {
        let immediate = self.get_immediate_subexpression_paths()
            .into_iter()
            .map(|x| x.into());
        let deferred = self.get_located_immediate_subexpressions()
            .into_iter()
            .map(|x| x.obj.get_subexpression_paths())
            .flatten();
        immediate.chain(deferred)
    }

    fn get_subexpression(&self,path: &TblSubexpressionInExpressionPath) -> Result<&UnassignedTblExpression<UnassignedRcCompoundTblExpression>, ()> { 
        let v = path.0.get(0).ok_or(())?;
        let inner = self.get_immediate_subexpression(v)?;
        if 1 == path.0.len() { Ok(inner) }
        else { inner.get_subexpressions_helper(path, 1) }
    }
}

mod from {
    use std::{rc::Rc, sync::Arc};

    use itertools::Itertools;

    use crate::expressions::types::{assigned::{RcTblExpression, compound::{TblExpressionCompound, rc::RcTblExpressionCompound}}, unassigned::{RcUnassignedTblExpression, UnassignedTblExpression, compound::{arc::UnassignedArcCompoundTblExpression, r#box::UnassignedBoxCompoundTblExpression, rc::UnassignedRcCompoundTblExpression}, variable::TblExpressionVariable}};

    impl <C: TblExpressionCompound> From<&C> for UnassignedRcCompoundTblExpression {
        fn from(value: &C) -> Self
            { value.as_slice().iter().map(|v| UnassignedTblExpression::from(v)).collect() }
    } impl TryInto<RcTblExpressionCompound> for &UnassignedRcCompoundTblExpression {
        type Error = TblExpressionVariable;
        fn try_into(self) -> Result<RcTblExpressionCompound, Self::Error> { self.0.iter()
            .map(|v| <&RcUnassignedTblExpression as TryInto<RcTblExpression>>::try_into(v))
            .try_collect()
        }
    }

    impl <const N: usize> From<[UnassignedTblExpression<Self>;N]> for UnassignedRcCompoundTblExpression
        { fn from(exprs: [UnassignedTblExpression<Self>;N]) -> Self { Self(Rc::new(exprs)) } }
    impl From<Box<[UnassignedTblExpression<Self>]>> for UnassignedRcCompoundTblExpression
        { fn from(exprs: Box<[UnassignedTblExpression<Self>]>) -> Self { Self(exprs.into()) } }
    impl From<Rc<[UnassignedTblExpression<Self>]>> for UnassignedRcCompoundTblExpression
        { fn from(exprs: Rc<[UnassignedTblExpression<Self>]>) -> Self { Self(exprs) } }
    impl From<Arc<[UnassignedTblExpression<Self>]>> for UnassignedRcCompoundTblExpression
        { fn from(exprs: Arc<[UnassignedTblExpression<Self>]>) -> Self { Self(Rc::from(exprs.into_iter().as_slice())) } }
    impl From<Vec<UnassignedTblExpression<Self>>> for UnassignedRcCompoundTblExpression
        { fn from(exprs: Vec<UnassignedTblExpression<Self>>) -> Self { Self(exprs.into()) } }
    impl FromIterator<UnassignedTblExpression<Self>> for UnassignedRcCompoundTblExpression
        { fn from_iter<T: IntoIterator<Item = UnassignedTblExpression<Self>>>(iter: T) -> Self { Self(iter.into_iter().collect()) } }
    
    impl From<&UnassignedBoxCompoundTblExpression> for UnassignedRcCompoundTblExpression {
        fn from(value: &UnassignedBoxCompoundTblExpression) -> Self {
            value.0.iter()
                .map(|i| match i {
                    UnassignedTblExpression::Atom(atomic) => UnassignedTblExpression::Atom(*atomic),
                    UnassignedTblExpression::Variable(variable) => UnassignedTblExpression::Variable(*variable),
                    UnassignedTblExpression::Compound(compound) => UnassignedTblExpression::Compound(UnassignedRcCompoundTblExpression::from(compound)),
                })
                .collect()
        }
    }
    impl From<&UnassignedArcCompoundTblExpression> for UnassignedRcCompoundTblExpression {
        fn from(value: &UnassignedArcCompoundTblExpression) -> Self {
            value.0.iter()
                .map(|i| match i {
                    UnassignedTblExpression::Atom(atomic) => UnassignedTblExpression::Atom(*atomic),
                    UnassignedTblExpression::Variable(variable) => UnassignedTblExpression::Variable(*variable),
                    UnassignedTblExpression::Compound(compound) => UnassignedTblExpression::Compound(UnassignedRcCompoundTblExpression::from(compound)),
                })
                .collect()
        }
    }
}
