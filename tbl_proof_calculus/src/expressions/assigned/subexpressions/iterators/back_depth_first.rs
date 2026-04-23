use crate::expressions::assigned::{TblExpression, at_path_enum::TblSubexpressionInExpressionEnum, compound::CompoundTblExpression, subexpressions::{TblSubexpressionInExpression, TblSubexpressionInExpressionPath, immediate::LocatedParentOfImmediateSubexpressions}};

pub struct BackDepthFirstTblExpressionIterator<'a,C: CompoundTblExpression> {
    exprs: Vec<TblSubexpressionInExpression<'a, C>>
}
impl <'a, C: CompoundTblExpression> BackDepthFirstTblExpressionIterator<'a,C> {
    pub fn new(expr: &'a TblExpression<C>) -> Self { Self { exprs: vec![
        TblSubexpressionInExpression { 
            obj: expr,
            path: TblSubexpressionInExpressionPath::default()
        }
    ]}}
}
impl <'a, C: CompoundTblExpression> Iterator for BackDepthFirstTblExpressionIterator<'a,C> {
    type Item = TblSubexpressionInExpressionEnum<'a,C>;
    fn next(&mut self) -> Option<Self::Item> {
        let next: TblSubexpressionInExpressionEnum<'a,C> = self.exprs.pop().map(|expr| expr.into())?;
        if let TblSubexpressionInExpressionEnum::Compound(compound) = next {
            self.exprs.extend(compound.clone().into_located_immediate_subexpressions());
            Some(TblSubexpressionInExpressionEnum::Compound(compound))
        } else { Some(next) }
    }
}
