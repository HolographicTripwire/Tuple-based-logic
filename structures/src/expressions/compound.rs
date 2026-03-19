use crate::expressions::{Expression, ExpressionAtPathEnum, subexpression::{ExpressionInExpressionPath, ParentOfSubexpressions, immediate::{ImmediateExpressionInExpressionPath, ParentOfImmediateSubexpressions}}};

#[derive(Debug,Clone,PartialEq,Eq,Hash)]
pub struct CompoundExpression(pub Vec<Expression>);

impl CompoundExpression {
    fn get_subexpressions_inner(&self,path: &ExpressionInExpressionPath, index: usize) -> Result<&Expression,()> {
        let immediate_path = path.0.get(index).ok_or(())?;
        let inner = self.get_immediate_subexpression(immediate_path)?;
        if index == path.0.len() { Ok(inner) }
        else { match inner {
            Expression::Atomic(_) => Err(()),
            Expression::Compound(compound) => compound.get_subexpressions_inner(path, index+1),
        }}
    }
}

impl ParentOfImmediateSubexpressions for CompoundExpression {
    fn get_immediate_subexpression_paths(&self) -> impl IntoIterator<Item = ImmediateExpressionInExpressionPath>
        { (0..self.0.len()).map(|x| x.into()) }

    fn get_immediate_subexpression(&self,path: &ImmediateExpressionInExpressionPath) -> Result< &Expression,()>
        { self.0.get(path.0).ok_or(()) }
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
        { self.get_subexpressions_inner(path, 0) }
}

mod from {
    use crate::expressions::{Expression, compound::CompoundExpression};

    impl From<Vec<Expression>> for CompoundExpression {
        fn from(value: Vec<Expression>) -> Self { Self(value) }
    }
}
