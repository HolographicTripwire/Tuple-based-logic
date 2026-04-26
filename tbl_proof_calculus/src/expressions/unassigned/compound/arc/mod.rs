use std::sync::Arc;

use crate::{expressions::unassigned::{UnassignedTblExpression, compound::UnassignedCompoundTblExpression, subexpressions::{ParentOfUnassignedSubexpressions, immediate::ParentOfImmediateUnassignedSubexpressions}}, expressions::assigned::{TblExpression, compound::{CompoundTblExpression, arc::ArcCompoundTblExpression}, subexpressions::{ParentOfSubexpressions, TblSubexpressionInExpressionPath, immediate::{ImmediateTblSubexpressionInExpressionPath, ParentOfImmediateSubexpressions}}}};

/// A compound unit in Tuple-Based Logic, which are used to build up [Propositions](Proposition)
#[derive(Debug,Clone,PartialEq,Eq,Hash)]
pub struct UnassignedArcCompoundTblExpression(pub Arc<[UnassignedTblExpression<UnassignedArcCompoundTblExpression>]>);
impl UnassignedCompoundTblExpression for UnassignedArcCompoundTblExpression {
    type InnerCompound = ArcCompoundTblExpression;
    
    fn len(&self) -> usize { self.0.len() }
    fn as_slice(&self) -> &[UnassignedTblExpression<Self>] { &self.0 }
    fn replace(&self, to_replace: &UnassignedTblExpression<Self>, replace_with: &UnassignedTblExpression<Self>) -> Self {
        self.0.iter()
            .map(|v| v.replace(to_replace, replace_with))
            .collect()
    }
    
    fn reverse_assign(&self, assigned: &Self) -> Result<crate::expressions::unassigned::assignments::TblExpressionAssignment<Self::InnerCompound>,()> {
        todo!()
    }
}

impl ParentOfImmediateUnassignedSubexpressions<UnassignedArcCompoundTblExpression> for UnassignedArcCompoundTblExpression {
    fn get_immediate_subexpression_paths(&self) -> impl IntoIterator<Item = ImmediateTblSubexpressionInExpressionPath>
        { (0..self.0.len()).map(|x| x.into()) }

    fn get_immediate_subexpression(&self,path: &ImmediateTblSubexpressionInExpressionPath) -> Result<&UnassignedTblExpression<UnassignedArcCompoundTblExpression>, ()>
        { self.0.get(path.0).ok_or(()) }
}

impl ParentOfUnassignedSubexpressions<UnassignedArcCompoundTblExpression> for UnassignedArcCompoundTblExpression {
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

    fn get_subexpression(&self,path: &TblSubexpressionInExpressionPath) -> Result<&UnassignedTblExpression<UnassignedArcCompoundTblExpression>, ()> { 
        let v = path.0.get(0).ok_or(())?;
        let inner = self.get_immediate_subexpression(v)?;
        if 1 == path.0.len() { Ok(inner) }
        else { inner.get_subexpressions_helper(path, 1) }
    }
}

mod from {
    use std::{rc::Rc, sync::Arc};

    use crate::{expressions::unassigned::{UnassignedTblExpression, compound::arc::UnassignedArcCompoundTblExpression}, expressions::assigned::{TblExpression, compound::{arc::ArcCompoundTblExpression, r#box::BoxCompoundTblExpression, rc::RcCompoundTblExpression}}};

    impl From<ArcCompoundTblExpression> for UnassignedArcCompoundTblExpression {
        fn from(value: ArcCompoundTblExpression) -> Self { 
            value.0
                .into_iter()
                .cloned()
                .map(|v| UnassignedTblExpression::from(v))
                .collect()
        }
    }
    impl TryInto<ArcCompoundTblExpression> for UnassignedArcCompoundTblExpression {
        type Error = usize;
    
        fn try_into(self) -> Result<ArcCompoundTblExpression, Self::Error> {
            self.0
                .into_iter()
                .cloned()
                .map(|v| v.try_into())
                .collect()
        }
    }


    impl <const N: usize> From<[UnassignedTblExpression<UnassignedArcCompoundTblExpression>;N]> for UnassignedArcCompoundTblExpression {
        fn from(exprs: [UnassignedTblExpression<UnassignedArcCompoundTblExpression>;N]) -> Self { Self(Arc::new(exprs)) }
    }
    impl From<Box<[UnassignedTblExpression<UnassignedArcCompoundTblExpression>]>> for UnassignedArcCompoundTblExpression {
        fn from(exprs: Box<[UnassignedTblExpression<UnassignedArcCompoundTblExpression>]>) -> Self { Self(exprs.into()) }
    }
    impl From<Rc<[UnassignedTblExpression<UnassignedArcCompoundTblExpression>]>> for UnassignedArcCompoundTblExpression {
        fn from(exprs: Rc<[UnassignedTblExpression<UnassignedArcCompoundTblExpression>]>) -> Self { Self(Arc::from(exprs.into_iter().as_slice())) }
    }
    impl From<Arc<[UnassignedTblExpression<UnassignedArcCompoundTblExpression>]>> for UnassignedArcCompoundTblExpression {
        fn from(exprs: Arc<[UnassignedTblExpression<UnassignedArcCompoundTblExpression>]>) -> Self { Self(exprs) }
    }
    impl From<Vec<UnassignedTblExpression<UnassignedArcCompoundTblExpression>>> for UnassignedArcCompoundTblExpression {
        fn from(exprs: Vec<UnassignedTblExpression<UnassignedArcCompoundTblExpression>>) -> Self { Self(exprs.into()) }
    }
    impl FromIterator<UnassignedTblExpression<UnassignedArcCompoundTblExpression>> for UnassignedArcCompoundTblExpression {
        fn from_iter<T: IntoIterator<Item = UnassignedTblExpression<UnassignedArcCompoundTblExpression>>>(iter: T) -> Self { Self(iter.into_iter().collect()) }
    }

    impl From<UnassignedBoxCompoundTblExpression> for UnassignedArcCompoundTblExpression {
        fn from(value: BoxCompoundTblExpression) -> Self {
            value.0.iter()
                .map(|i| match i {
                    UnassignedTblExpression::Atomic(atomic) => TblExpression::Atomic(atomic.clone()),
                    UnassignedTblExpression::Compound(compound) => TblExpression::Compound(UnassignedArcCompoundTblExpression::from(compound.clone())),
                })
                .collect()
        }
    }
    impl From<UnassignedRcCompoundTblExpression> for UnassignedArcCompoundTblExpression {
        fn from(value: UnassignedRcCompoundTblExpression) -> Self {
            value.0.iter()
                .map(|i| match i {
                    TblExpression::Atomic(atomic) => UnassignedTblExpression::Atomic(atomic.clone()),
                    TblExpression::Compound(compound) => UnassignedTblExpression::Compound(UnassignedArcCompoundTblExpression::from(compound.clone())),
                })
                .collect()
        }
    }
}
