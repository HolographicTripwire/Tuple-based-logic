use crate::expressions::{paths::{TblSubexpressionInExpressionPath, immediate::ImmediateTblSubexpressionInExpressionPath}, types::assigned::{TblExpression, compound::CompoundTblExpression, subexpressions::{ParentOfImmediateSubexpressions, ParentOfSubexpressions}}};

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

    use crate::expressions::types::assigned::{TblExpression, compound::{arc::ArcCompoundTblExpression, r#box::BoxCompoundTblExpression, rc::RcCompoundTblExpression}};

    impl <const N: usize> From<[TblExpression<Self>;N]> for BoxCompoundTblExpression
        { fn from(exprs: [TblExpression<Self>;N]) -> Self { Self(Box::new(exprs)) } }
    impl From<Box<[TblExpression<Self>]>> for BoxCompoundTblExpression
        { fn from(exprs: Box<[TblExpression<Self>]>) -> Self { Self(exprs) } }
    impl From<Rc<[TblExpression<Self>]>> for BoxCompoundTblExpression
        { fn from(exprs: Rc<[TblExpression<Self>]>) -> Self { Self(Box::from(exprs.into_iter().as_slice())) } }
    impl From<Arc<[TblExpression<Self>]>> for BoxCompoundTblExpression
        { fn from(exprs: Arc<[TblExpression<Self>]>) -> Self { Self(Box::from(exprs.into_iter().as_slice())) } }
    impl From<Vec<TblExpression<Self>>> for BoxCompoundTblExpression
        { fn from(exprs: Vec<TblExpression<Self>>) -> Self { Self(exprs.into()) } }
    impl FromIterator<TblExpression<Self>> for BoxCompoundTblExpression
        { fn from_iter<T: IntoIterator<Item = TblExpression<Self>>>(iter: T) -> Self { Self(iter.into_iter().collect()) } }

    impl <'a> From<&'a Self> for BoxCompoundTblExpression 
        { fn from(value: &'a Self) -> Self { value.clone() } }
    impl From<&RcCompoundTblExpression> for BoxCompoundTblExpression {
        fn from(value: &RcCompoundTblExpression) -> Self
            { value.0.iter() .map(|i| i.transmute_compound()).collect() }
    } impl From<&ArcCompoundTblExpression> for BoxCompoundTblExpression {
        fn from(value: &ArcCompoundTblExpression) -> Self 
            { value.0.iter().map(|i| i.transmute_compound()).collect() }
    }
}
