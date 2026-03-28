use crate::structures::expressions::{TblExpression, at_path_enum::ExpressionAtPathEnum, compound::CompoundTblExpression, subexpressions::{ParentOfSubexpressions, SubexpressionInExpressionPath, immediate::{ImmediateSubexpressionInExpressionPath, ParentOfImmediateSubexpressions}}};

#[derive(Debug,Clone,PartialEq,Eq,Hash)]
pub struct RefCompoundTblExpression<'a>(pub &'a [TblExpression<RefCompoundTblExpression<'a>>]);
impl <'a> CompoundTblExpression for RefCompoundTblExpression<'a> {
    fn len(&self) -> usize { self.0.len() }
}

impl <'a> ParentOfImmediateSubexpressions<RefCompoundTblExpression<'a>> for RefCompoundTblExpression<'a> {
    fn get_immediate_subexpression_paths(&self) -> impl IntoIterator<Item = ImmediateSubexpressionInExpressionPath>
        { (0..self.0.len()).map(|x| x.into()) }

    fn get_immediate_subexpression(&self,path: &ImmediateSubexpressionInExpressionPath) -> Result<&TblExpression<RefCompoundTblExpression<'a>>,()>
        { self.0.get(path.0).ok_or(()) }
}

impl <'a> RefCompoundTblExpression<'a> {
    fn get_subexpressions_helper(&self,path: &SubexpressionInExpressionPath, index: usize) -> Result<&TblExpression<RefCompoundTblExpression<'a>>,()> {
        let immediate_path = path.0.get(index).ok_or(())?;
        let inner = self.get_immediate_subexpression(immediate_path)?;
        if index == path.0.len() { Ok(inner) }
        else { match inner {
            TblExpression::Atomic(_) => Err(()),
            TblExpression::Compound(compound) => compound.get_subexpressions_helper(path, index+1),
        }}
    }
}
impl <'a> ParentOfSubexpressions<RefCompoundTblExpression<'a>> for RefCompoundTblExpression<'a> {
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

    fn get_subexpression(&self,path: &SubexpressionInExpressionPath) -> Result<&TblExpression<RefCompoundTblExpression<'a>>,()>
        { self.get_subexpressions_helper(path, 0) }
}

mod from {
    use crate::structures::expressions::{TblExpression, compound::r#ref::RefCompoundTblExpression};

    impl <'a> FromIterator<&'a TblExpression<RefCompoundTblExpression<'a>>> for RefCompoundTblExpression<'a> {
        fn from_iter<T: IntoIterator<Item = &'a TblExpression<RefCompoundTblExpression<'a>>>>(iter: T) -> Self { Self(iter.into_iter()) }
    }
}