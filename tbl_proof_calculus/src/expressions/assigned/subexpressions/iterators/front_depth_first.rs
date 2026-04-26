use crate::expressions::assigned::{TblExpression, at_path_enum::TblSubexpressionInExpressionEnum, compound::CompoundTblExpression, subexpressions::{TblSubexpressionInExpression, TblSubexpressionInExpressionPath, immediate::LocatedParentOfImmediateSubexpressions}};

pub struct FrontDepthFirstTblSubexpressionIterator<'a,C: CompoundTblExpression> {
    exprs: Vec<&'a TblExpression<C>>
}
impl <'a, C: CompoundTblExpression> FrontDepthFirstTblSubexpressionIterator<'a,C> {
    pub fn new(expr: &'a TblExpression<C>) -> Self { Self { exprs: vec![expr]}}
}
impl <'a, C: CompoundTblExpression> Iterator for FrontDepthFirstTblSubexpressionIterator<'a,C> {
    type Item = &'a TblExpression<C>;
    fn next(&mut self) -> Option<Self::Item> {
        let next = self.exprs.pop()?;
        if let TblSubexpressionInExpressionEnum::Compound(compound) = next.into() {
            self.exprs.extend(compound.clone().get_immediate_subexpressions().into_iter().rev());
        }
        Some(next)
    }
}

pub struct FrontDepthFirstLocatedTblSubexpressionIterator<'a,C: CompoundTblExpression> {
    exprs: Vec<TblSubexpressionInExpression<'a, C>>
}
impl <'a, C: CompoundTblExpression> FrontDepthFirstLocatedTblSubexpressionIterator<'a,C> {
    pub fn new(expr: &'a TblExpression<C>) -> Self { Self { exprs: vec![
        TblSubexpressionInExpression { 
            obj: expr,
            path: TblSubexpressionInExpressionPath::default()
        }
    ]}}
}
impl <'a, C: CompoundTblExpression> Iterator for FrontDepthFirstLocatedTblSubexpressionIterator<'a,C> {
    type Item = TblSubexpressionInExpression<'a,C>;
    fn next(&mut self) -> Option<Self::Item> {
        let next = self.exprs.pop()?;
        let result = next.clone();
        if let TblSubexpressionInExpressionEnum::Compound(compound) = next.into() {
            self.exprs.extend(compound.clone().into_located_immediate_subexpressions().into_iter().rev());
        }
        Some(result)
    }
}
