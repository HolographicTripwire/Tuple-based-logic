use crate::expressions::{assigned::subexpressions::TblSubexpressionInExpressionPath, unassigned::{UnassignedTblExpression, at_path_enum::UnassignedTblSubexpressionInExpressionEnum, compound::UnassignedCompoundTblExpression, subexpressions::{UnassignedTblSubexpressionInExpression, immediate::LocatedParentOfImmediateUnassignedSubexpressions}}};

pub struct BackDepthFirstUnassignedTblExpressionIterator<'a,C: UnassignedCompoundTblExpression> {
    exprs: Vec<UnassignedTblSubexpressionInExpression<'a, C>>
}
impl <'a, C: UnassignedCompoundTblExpression> BackDepthFirstUnassignedTblExpressionIterator<'a,C> {
    pub fn new(expr: &'a UnassignedTblExpression<C>) -> Self { Self { exprs: vec![
        UnassignedTblSubexpressionInExpression { 
            obj: expr,
            path: TblSubexpressionInExpressionPath::default()
        }
    ]}}
}
impl <'a, C: UnassignedCompoundTblExpression> Iterator for BackDepthFirstUnassignedTblExpressionIterator<'a,C> {
    type Item = UnassignedTblSubexpressionInExpressionEnum<'a,C>;
    fn next(&mut self) -> Option<Self::Item> {
        let next: UnassignedTblSubexpressionInExpressionEnum<'a,C> = self.exprs.pop().map(|expr| expr.into())?;
        if let UnassignedTblSubexpressionInExpressionEnum::Compound(compound) = next {
            self.exprs.extend(compound.clone().into_located_immediate_subexpressions());
            Some(UnassignedTblSubexpressionInExpressionEnum::Compound(compound))
        } else { Some(next) }
    }
}
