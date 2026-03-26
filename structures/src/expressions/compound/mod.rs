use std::sync::Arc;

use crate::expressions::{Expression, at_path_enum::ExpressionAtPathEnum, subexpression::{ExpressionInExpressionPath, ParentOfSubexpressions, immediate::{ImmediateExpressionInExpressionPath, ParentOfImmediateSubexpressions}}};

// trait CompoundExpression: ParentOfImmediateSubexpressions + ParentOfSubexpressions {
//     fn replace(&self, to_replace: ) {

//     }
// }

#[derive(Debug,Clone,PartialEq,Eq,Hash)]
pub struct CompoundExpression(pub Arc<[Expression]>);

impl CompoundExpression {
    pub fn replace(&self, to_replace: &Expression, replace_with: &Expression) -> Self {
        Self(self.0.iter()
            .map(|expr|
                if expr == to_replace { replace_with.clone() }
                else { expr.replace(to_replace,replace_with) }
            ).collect()
        )
    }
}

impl ParentOfImmediateSubexpressions for CompoundExpression {
    fn get_immediate_subexpression_paths(&self) -> impl IntoIterator<Item = ImmediateExpressionInExpressionPath>
        { (0..self.0.len()).map(|x| x.into()) }

    fn get_immediate_subexpression(&self,path: &ImmediateExpressionInExpressionPath) -> Result< &Expression,()>
        { self.0.get(path.0).ok_or(()) }
}

impl CompoundExpression {
    fn get_subexpressions_helper(&self,path: &ExpressionInExpressionPath, index: usize) -> Result<&Expression,()> {
        let immediate_path = path.0.get(index).ok_or(())?;
        let inner = self.get_immediate_subexpression(immediate_path)?;
        if index == path.0.len() { Ok(inner) }
        else { match inner {
            Expression::Atomic(_) => Err(()),
            Expression::Compound(compound) => compound.get_subexpressions_helper(path, index+1),
        }}
    }
}
impl ParentOfSubexpressions for CompoundExpression {
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

    fn get_subexpression(&self,path: &ExpressionInExpressionPath) -> Result<&Expression,()>
        { self.get_subexpressions_helper(path, 0) }
}

mod from {
    use std::sync::Arc;

    use crate::expressions::{Expression, compound::CompoundExpression};

    impl <const N: usize> From<[Expression;N]> for CompoundExpression {
        fn from(exprs: [Expression;N]) -> Self { Self(Arc::new(exprs)) }
    }
    impl From<Box<[Expression]>> for CompoundExpression {
        fn from(exprs: Box<[Expression]>) -> Self { Self(exprs.into()) }
    }
    impl From<Arc<[Expression]>> for CompoundExpression {
        fn from(exprs: Arc<[Expression]>) -> Self { Self(exprs) }
    }
    impl From<Vec<Expression>> for CompoundExpression {
        fn from(exprs: Vec<Expression>) -> Self { Self(exprs.into()) }
    }    
}
