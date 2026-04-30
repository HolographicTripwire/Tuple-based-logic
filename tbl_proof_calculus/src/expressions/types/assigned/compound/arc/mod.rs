use std::sync::Arc;

use crate::expressions::{paths::{TblSubexpressionInExpressionPath, immediate::ImmediateTblSubexpressionInExpressionPath}, types::assigned::{TblExpression, compound::CompoundTblExpression, subexpressions::{ParentOfImmediateSubexpressions, ParentOfSubexpressions}}};

/// A compound unit in Tuple-Based Logic, which are used to build up [Propositions](Proposition)
#[derive(Debug,Clone,PartialEq,Eq,Hash)]
pub struct ArcCompoundTblExpression(pub Arc<[TblExpression<ArcCompoundTblExpression>]>);
impl CompoundTblExpression for ArcCompoundTblExpression {
    fn len(&self) -> usize { self.0.len() }
    fn as_slice(&self) -> &[TblExpression<Self>] { &self.0 }
    fn replace(&self, to_replace: &TblExpression<Self>, replace_with: &TblExpression<Self>) -> Self {
        self.0.iter()
            .map(|v| v.replace(to_replace, replace_with))
            .collect()
    }
}

impl ParentOfImmediateSubexpressions<ArcCompoundTblExpression> for ArcCompoundTblExpression {
    fn get_immediate_subexpression_paths(&self) -> impl IntoIterator<Item = ImmediateTblSubexpressionInExpressionPath>
        { (0..self.0.len()).map(|x| x.into()) }

    fn get_immediate_subexpression(&self,path: &ImmediateTblSubexpressionInExpressionPath) -> Result<&TblExpression<ArcCompoundTblExpression>,()>
        { self.0.get(path.0).ok_or(()) }
}

impl ParentOfSubexpressions<ArcCompoundTblExpression> for ArcCompoundTblExpression {
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

    fn get_subexpression(&self,path: &TblSubexpressionInExpressionPath) -> Result<&TblExpression<ArcCompoundTblExpression>,()> { 
        let v = path.0.get(0).ok_or(())?;
        let inner = self.get_immediate_subexpression(v)?;
        if 1 == path.0.len() { Ok(inner) }
        else { inner.get_subexpressions_helper(path, 1) }
    }
}

mod from {
    use std::{rc::Rc, sync::Arc};

    use crate::expressions::types::assigned::{TblExpression, compound::{arc::ArcCompoundTblExpression, r#box::BoxCompoundTblExpression, rc::RcCompoundTblExpression}};

    impl <const N: usize> From<[TblExpression<ArcCompoundTblExpression>;N]> for ArcCompoundTblExpression {
        fn from(exprs: [TblExpression<ArcCompoundTblExpression>;N]) -> Self { Self(Arc::new(exprs)) }
    }
    impl From<Box<[TblExpression<ArcCompoundTblExpression>]>> for ArcCompoundTblExpression {
        fn from(exprs: Box<[TblExpression<ArcCompoundTblExpression>]>) -> Self { Self(exprs.into()) }
    }
    impl From<Rc<[TblExpression<ArcCompoundTblExpression>]>> for ArcCompoundTblExpression {
        fn from(exprs: Rc<[TblExpression<ArcCompoundTblExpression>]>) -> Self { Self(Arc::from(exprs.into_iter().as_slice())) }
    }
    impl From<Arc<[TblExpression<ArcCompoundTblExpression>]>> for ArcCompoundTblExpression {
        fn from(exprs: Arc<[TblExpression<ArcCompoundTblExpression>]>) -> Self { Self(exprs) }
    }
    impl From<Vec<TblExpression<ArcCompoundTblExpression>>> for ArcCompoundTblExpression {
        fn from(exprs: Vec<TblExpression<ArcCompoundTblExpression>>) -> Self { Self(exprs.into()) }
    }
    impl FromIterator<TblExpression<ArcCompoundTblExpression>> for ArcCompoundTblExpression {
        fn from_iter<T: IntoIterator<Item = TblExpression<ArcCompoundTblExpression>>>(iter: T) -> Self { Self(iter.into_iter().collect()) }
    }

    impl From<&BoxCompoundTblExpression> for ArcCompoundTblExpression {
        fn from(value: &BoxCompoundTblExpression) -> Self {
            value.0.iter()
                .map(|i| i.transmute_compound())
                .collect()
        }
    }
    impl From<&RcCompoundTblExpression> for ArcCompoundTblExpression {
        fn from(value: &RcCompoundTblExpression) -> Self {
            value.0.iter()
                .map(|i| i.transmute_compound())
                .collect()
        }
    }
}
