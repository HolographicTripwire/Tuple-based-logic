use std::sync::Arc;

use crate::expressions::{TblExpression, at_path_enum::ExpressionAtPathEnum, subexpression::{ExpressionInExpressionPath, ParentOfSubexpressions, immediate::{ImmediateExpressionInExpressionPath, ParentOfImmediateSubexpressions}}};

// trait CompoundExpression: ParentOfImmediateSubexpressions + ParentOfSubexpressions {
//     fn replace(&self, to_replace: ) {

//     }
// }

/// A compound unit in Tuple-Based Logic, which are used to build up [Propositions](Proposition)
#[derive(Debug,Clone,PartialEq,Eq,Hash)]
pub struct CompoundTblExpression(pub Arc<[TblExpression]>);

impl CompoundTblExpression {
    pub fn replace(&self, to_replace: &TblExpression, replace_with: &TblExpression) -> Self {
        Self(self.0.iter()
            .map(|expr|
                if expr == to_replace { replace_with.clone() }
                else { expr.replace(to_replace,replace_with) }
            ).collect()
        )
    }
}

impl ParentOfImmediateSubexpressions for CompoundTblExpression {
    fn get_immediate_subexpression_paths(&self) -> impl IntoIterator<Item = ImmediateExpressionInExpressionPath>
        { (0..self.0.len()).map(|x| x.into()) }

    fn get_immediate_subexpression(&self,path: &ImmediateExpressionInExpressionPath) -> Result<&TblExpression,()>
        { self.0.get(path.0).ok_or(()) }
}

impl CompoundTblExpression {
    fn get_subexpressions_helper(&self,path: &ExpressionInExpressionPath, index: usize) -> Result<&TblExpression,()> {
        let immediate_path = path.0.get(index).ok_or(())?;
        let inner = self.get_immediate_subexpression(immediate_path)?;
        if index == path.0.len() { Ok(inner) }
        else { match inner {
            TblExpression::Atomic(_) => Err(()),
            TblExpression::Compound(compound) => compound.get_subexpressions_helper(path, index+1),
        }}
    }
}
impl ParentOfSubexpressions for CompoundTblExpression {
    fn get_subexpression_paths(&self) -> impl IntoIterator<Item = ExpressionInExpressionPath>  {
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

    fn get_subexpression(&self,path: &ExpressionInExpressionPath) -> Result<&TblExpression,()>
        { self.get_subexpressions_helper(path, 0) }
}

mod from {
    use std::sync::Arc;

    use crate::expressions::{TblExpression, compound::CompoundTblExpression};

    impl <const N: usize> From<[TblExpression;N]> for CompoundTblExpression {
        fn from(exprs: [TblExpression;N]) -> Self { Self(Arc::new(exprs)) }
    }
    impl From<Box<[TblExpression]>> for CompoundTblExpression {
        fn from(exprs: Box<[TblExpression]>) -> Self { Self(exprs.into()) }
    }
    impl From<Arc<[TblExpression]>> for CompoundTblExpression {
        fn from(exprs: Arc<[TblExpression]>) -> Self { Self(exprs) }
    }
    impl From<Vec<TblExpression>> for CompoundTblExpression {
        fn from(exprs: Vec<TblExpression>) -> Self { Self(exprs.into()) }
    }    
}
