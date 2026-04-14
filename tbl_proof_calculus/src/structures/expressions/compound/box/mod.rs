use crate::structures::expressions::{TblExpression, compound::CompoundTblExpression, subexpressions::{ParentOfSubexpressions, TblSubexpressionInExpressionPath, immediate::{ImmediateTblSubexpressionInExpressionPath, ParentOfImmediateSubexpressions}}};

#[derive(Debug,Clone,PartialEq,Eq,Hash)]
pub struct BoxCompoundTblExpression(pub Box<[TblExpression<BoxCompoundTblExpression>]>);
impl CompoundTblExpression for BoxCompoundTblExpression {
    fn len(&self) -> usize { self.0.len() }
    fn as_slice(&self) -> &[TblExpression<Self>] { &self.0 }
    fn replace(&self, to_replace: &TblExpression<Self>, replace_with: &TblExpression<Self>) -> Self {
        self.0.iter()
            .map(|v| v.replace(to_replace, replace_with))
            .collect()
    }
}

impl ParentOfImmediateSubexpressions<BoxCompoundTblExpression> for BoxCompoundTblExpression {
    fn get_immediate_subexpression_paths(&self) -> impl IntoIterator<Item = ImmediateTblSubexpressionInExpressionPath>
        { (0..self.0.len()).map(|x| x.into()) }
    fn get_immediate_subexpression(&self,path: &ImmediateTblSubexpressionInExpressionPath) -> Result<&TblExpression<BoxCompoundTblExpression>,()>
        { self.0.get(path.0).ok_or(()) }
}


impl ParentOfSubexpressions<BoxCompoundTblExpression> for BoxCompoundTblExpression {
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

    fn get_subexpression(&self,path: &TblSubexpressionInExpressionPath) -> Result<&TblExpression<BoxCompoundTblExpression>,()> { 
        let v = path.0.get(0).ok_or(())?;
        let inner = self.get_immediate_subexpression(v)?;
        if 1 == path.0.len() { Ok(inner) }
        else { inner.get_subexpressions_helper(path, 1) }
    }
}

mod from {
    use std::{rc::Rc, sync::Arc};

    use crate::structures::expressions::{TblExpression, compound::{arc::ArcCompoundTblExpression, r#box::BoxCompoundTblExpression, rc::RcCompoundTblExpression}};

    impl <const N: usize> From<[TblExpression<BoxCompoundTblExpression>;N]> for BoxCompoundTblExpression {
        fn from(exprs: [TblExpression<BoxCompoundTblExpression>;N]) -> Self { Self(Box::new(exprs)) }
    }
    impl From<Box<[TblExpression<BoxCompoundTblExpression>]>> for BoxCompoundTblExpression {
        fn from(exprs: Box<[TblExpression<BoxCompoundTblExpression>]>) -> Self { Self(exprs) }
    }
    impl From<Rc<[TblExpression<BoxCompoundTblExpression>]>> for BoxCompoundTblExpression {
        fn from(exprs: Rc<[TblExpression<BoxCompoundTblExpression>]>) -> Self { Self(Box::from(exprs.into_iter().as_slice())) }
    }
    impl From<Arc<[TblExpression<BoxCompoundTblExpression>]>> for BoxCompoundTblExpression {
        fn from(exprs: Arc<[TblExpression<BoxCompoundTblExpression>]>) -> Self { Self(Box::from(exprs.into_iter().as_slice())) }
    }
    impl From<Vec<TblExpression<BoxCompoundTblExpression>>> for BoxCompoundTblExpression {
        fn from(exprs: Vec<TblExpression<BoxCompoundTblExpression>>) -> Self { Self(exprs.into()) }
    }
    impl FromIterator<TblExpression<BoxCompoundTblExpression>> for BoxCompoundTblExpression {
        fn from_iter<T: IntoIterator<Item = TblExpression<BoxCompoundTblExpression>>>(iter: T) -> Self { Self(iter.into_iter().collect()) }
    }

    impl From<RcCompoundTblExpression> for BoxCompoundTblExpression {
        fn from(value: RcCompoundTblExpression) -> Self {
            value.0.iter()
                .map(|i| match i {
                    TblExpression::Atomic(atomic) => TblExpression::Atomic(atomic.clone()),
                    TblExpression::Compound(compound) => TblExpression::Compound(BoxCompoundTblExpression::from(compound.clone())),
                })
                .collect()
        }
    }
    impl From<ArcCompoundTblExpression> for BoxCompoundTblExpression {
        fn from(value: ArcCompoundTblExpression) -> Self {
            value.0.iter()
                .map(|i| match i {
                    TblExpression::Atomic(atomic) => TblExpression::Atomic(atomic.clone()),
                    TblExpression::Compound(compound) => TblExpression::Compound(BoxCompoundTblExpression::from(compound.clone())),
                })
                .collect()
        }
    }
}
