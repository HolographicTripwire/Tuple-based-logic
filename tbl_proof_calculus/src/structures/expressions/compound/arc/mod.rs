use std::sync::Arc;

use crate::structures::expressions::{TblExpression, at_path_enum::ExpressionAtPathEnum, compound::CompoundTblExpression, subexpressions::{ParentOfSubexpressions, SubexpressionInExpressionPath, immediate::{ImmediateSubexpressionInExpressionPath, ParentOfImmediateSubexpressions}}};

/// A compound unit in Tuple-Based Logic, which are used to build up [Propositions](Proposition)
#[derive(Debug,Clone,PartialEq,Eq,Hash)]
pub struct ArcCompoundTblExpression(pub Arc<[TblExpression<ArcCompoundTblExpression>]>);
impl CompoundTblExpression for ArcCompoundTblExpression {
    fn len(&self) -> usize { self.0.len() }
    
    fn replace(&self, to_replace: &TblExpression<Self>, replace_with: &TblExpression<Self>) -> Self {
        self.0.iter()
            .map(|v| v.replace(to_replace, replace_with))
            .collect()
    }
}

impl ParentOfImmediateSubexpressions<ArcCompoundTblExpression> for ArcCompoundTblExpression {
    fn get_immediate_subexpression_paths(&self) -> impl IntoIterator<Item = ImmediateSubexpressionInExpressionPath>
        { (0..self.0.len()).map(|x| x.into()) }

    fn get_immediate_subexpression(&self,path: &ImmediateSubexpressionInExpressionPath) -> Result<&TblExpression<ArcCompoundTblExpression>,()>
        { self.0.get(path.0).ok_or(()) }
}

impl ArcCompoundTblExpression {
    fn get_subexpressions_helper(&self,path: &SubexpressionInExpressionPath, index: usize) -> Result<&TblExpression<ArcCompoundTblExpression>,()> {
        let immediate_path = path.0.get(index).ok_or(())?;
        let inner = self.get_immediate_subexpression(immediate_path)?;
        if index == path.0.len() { Ok(inner) }
        else { match inner {
            TblExpression::Atomic(_) => Err(()),
            TblExpression::Compound(compound) => compound.get_subexpressions_helper(path, index+1),
        }}
    }
}
impl ParentOfSubexpressions<ArcCompoundTblExpression> for ArcCompoundTblExpression {
    fn get_subexpression_paths(&self) -> impl IntoIterator<Item = SubexpressionInExpressionPath>  {
        let immediate = self.get_immediate_subexpression_paths()
            .into_iter()
            .map(|x| x.into());
        let deferred = self.get_located_immediate_subexpressions()
            .into_iter()
            .map(|x| match x.into() {
                ExpressionAtPathEnum::Atomic(_) => vec![],
                ExpressionAtPathEnum::Compound(compound) => compound.obj
                    .get_subexpression_paths()
                    .into_iter()
                    .map(|p| (compound.path,p).into())
                    .collect()
                }
            ).flatten();
        immediate.chain(deferred)
    }

    fn get_subexpression(&self,path: &SubexpressionInExpressionPath) -> Result<&TblExpression<ArcCompoundTblExpression>,()>
        { self.get_subexpressions_helper(path, 0) }
}

mod from {
    use std::{rc::Rc, sync::Arc};

    use crate::structures::expressions::{TblExpression, compound::{arc::ArcCompoundTblExpression, r#box::BoxCompoundTblExpression, rc::RcCompoundTblExpression}};

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

    impl From<BoxCompoundTblExpression> for ArcCompoundTblExpression {
        fn from(value: BoxCompoundTblExpression) -> Self {
            value.0.iter()
                .map(|i| match i {
                    TblExpression::Atomic(atomic) => TblExpression::Atomic(atomic.clone()),
                    TblExpression::Compound(compound) => TblExpression::Compound(ArcCompoundTblExpression::from(compound.clone())),
                })
                .collect()
        }
    }
    impl From<RcCompoundTblExpression> for ArcCompoundTblExpression {
        fn from(value: RcCompoundTblExpression) -> Self {
            value.0.iter()
                .map(|i| match i {
                    TblExpression::Atomic(atomic) => TblExpression::Atomic(atomic.clone()),
                    TblExpression::Compound(compound) => TblExpression::Compound(ArcCompoundTblExpression::from(compound.clone())),
                })
                .collect()
        }
    }
}
