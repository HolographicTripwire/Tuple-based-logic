use std::rc::Rc;

use crate::structures::expressions::{TblExpression, compound::CompoundTblExpression, subexpressions::{ParentOfSubexpressions, TblSubexpressionInExpressionPath, immediate::{ImmediateTblSubexpressionInExpressionPath, ParentOfImmediateSubexpressions}}};

#[derive(Debug,Clone,PartialEq,Eq,Hash)]
pub struct RcCompoundTblExpression(pub Rc<[TblExpression<RcCompoundTblExpression>]>);
impl CompoundTblExpression for RcCompoundTblExpression {
    fn len(&self) -> usize { self.0.len() }
    fn as_slice(&self) -> &[TblExpression<Self>] { &self.0 }
    fn replace(&self, to_replace: &TblExpression<Self>, replace_with: &TblExpression<Self>) -> Self {
        self.0.iter()
            .map(|v| v.replace(to_replace, replace_with))
            .collect()
    }
}

impl ParentOfImmediateSubexpressions<RcCompoundTblExpression> for RcCompoundTblExpression {
    fn get_immediate_subexpression_paths(&self) -> impl IntoIterator<Item = ImmediateTblSubexpressionInExpressionPath>
        { (0..self.0.len()).map(|x| x.into()) }

    fn get_immediate_subexpression(&self,path: &ImmediateTblSubexpressionInExpressionPath) -> Result<&TblExpression<RcCompoundTblExpression>,()>
        { self.0.get(path.0).ok_or(()) }
}

impl ParentOfSubexpressions<RcCompoundTblExpression> for RcCompoundTblExpression {
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

    fn get_subexpression(&self,path: &TblSubexpressionInExpressionPath) -> Result<&TblExpression<RcCompoundTblExpression>,()> { 
        let v = path.0.get(0).ok_or(())?;
        let inner = self.get_immediate_subexpression(v)?;
        if 1 == path.0.len() { Ok(inner) }
        else { inner.get_subexpressions_helper(path, 1) }
    }
}

mod from {
    use std::{rc::Rc, sync::Arc};

    use crate::structures::expressions::{TblExpression, compound::{arc::ArcCompoundTblExpression, r#box::BoxCompoundTblExpression, rc::RcCompoundTblExpression}};

    impl <const N: usize> From<[TblExpression<RcCompoundTblExpression>;N]> for RcCompoundTblExpression {
        fn from(exprs: [TblExpression<RcCompoundTblExpression>;N]) -> Self { Self(Rc::new(exprs)) }
    }
    impl From<Box<[TblExpression<RcCompoundTblExpression>]>> for RcCompoundTblExpression {
        fn from(exprs: Box<[TblExpression<RcCompoundTblExpression>]>) -> Self { Self(Rc::from(exprs)) }
    }
    impl From<Rc<[TblExpression<RcCompoundTblExpression>]>> for RcCompoundTblExpression {
        fn from(exprs: Rc<[TblExpression<RcCompoundTblExpression>]>) -> Self { Self(exprs) }
    }
    impl From<Arc<[TblExpression<RcCompoundTblExpression>]>> for RcCompoundTblExpression {
        fn from(exprs: Arc<[TblExpression<RcCompoundTblExpression>]>) -> Self { Self(Rc::from(exprs.into_iter().as_slice())) }
    }
    impl From<Vec<TblExpression<RcCompoundTblExpression>>> for RcCompoundTblExpression {
        fn from(exprs: Vec<TblExpression<RcCompoundTblExpression>>) -> Self { Self(exprs.into()) }
    }
    impl FromIterator<TblExpression<RcCompoundTblExpression>> for RcCompoundTblExpression {
        fn from_iter<T: IntoIterator<Item = TblExpression<RcCompoundTblExpression>>>(iter: T) -> Self { Self(iter.into_iter().collect()) }
    }

    impl From<BoxCompoundTblExpression> for RcCompoundTblExpression {
        fn from(value: BoxCompoundTblExpression) -> Self {
            value.0.iter()
                .map(|i| match i {
                    TblExpression::Atomic(atomic) => TblExpression::Atomic(atomic.clone()),
                    TblExpression::Compound(compound) => TblExpression::Compound(RcCompoundTblExpression::from(compound.clone())),
                })
                .collect()
        }
    }
    impl From<ArcCompoundTblExpression> for RcCompoundTblExpression {
        fn from(value: ArcCompoundTblExpression) -> Self {
            value.0.iter()
                .map(|i| match i {
                    TblExpression::Atomic(atomic) => TblExpression::Atomic(atomic.clone()),
                    TblExpression::Compound(compound) => TblExpression::Compound(RcCompoundTblExpression::from(compound.clone())),
                })
                .collect()
        }
    }
}
