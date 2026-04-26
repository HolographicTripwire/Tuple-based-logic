use crate::expressions::{assigned::subexpressions::TblSubexpressionInExpressionPath, unassigned::{UnassignedTblExpression, at_path_enum::UnassignedTblSubexpressionInExpressionEnum, compound::UnassignedCompoundTblExpression, subexpressions::{UnassignedTblSubexpressionInExpression, immediate::LocatedParentOfImmediateUnassignedSubexpressions}}};

pub struct FrontDepthFirstUnassignedTblSubexpressionIterator<'a,C: UnassignedCompoundTblExpression> {
    exprs: Vec<&'a UnassignedTblExpression<C>>
}
impl <'a, C: UnassignedCompoundTblExpression> FrontDepthFirstUnassignedTblSubexpressionIterator<'a,C> {
    pub fn new(expr: &'a UnassignedTblExpression<C>) -> Self { Self { exprs: vec![expr] }}
}
impl <'a, C: UnassignedCompoundTblExpression> Iterator for FrontDepthFirstUnassignedTblSubexpressionIterator<'a,C> {
    type Item = &'a UnassignedTblExpression<C>;
    fn next(&mut self) -> Option<Self::Item> {
        let next = self.exprs.pop()?;
        let result = next;
        if let UnassignedTblSubexpressionInExpressionEnum::Compound(compound) = next.into() {
            self.exprs.extend(compound.get_immediate_subexpressions().into_iter().rev());
        }
        Some(result)
    }
}

pub struct FrontDepthFirstLocatedUnassignedTblSubexpressionIterator<'a,C: UnassignedCompoundTblExpression> {
    exprs: Vec<UnassignedTblSubexpressionInExpression<'a, C>>
}
impl <'a, C: UnassignedCompoundTblExpression> FrontDepthFirstLocatedUnassignedTblSubexpressionIterator<'a,C> {
    pub fn new(expr: &'a UnassignedTblExpression<C>) -> Self { Self { exprs: vec![
        UnassignedTblSubexpressionInExpression { 
            obj: expr,
            path: TblSubexpressionInExpressionPath::default()
        }
    ]}}
}
impl <'a, C: UnassignedCompoundTblExpression> Iterator for FrontDepthFirstLocatedUnassignedTblSubexpressionIterator<'a,C> {
    type Item = UnassignedTblSubexpressionInExpression<'a,C>;
    fn next(&mut self) -> Option<Self::Item> {
        let next = self.exprs.pop()?;
        let result = next.clone();
        if let UnassignedTblSubexpressionInExpressionEnum::Compound(compound) = next.into() {
            self.exprs.extend(compound.clone().into_located_immediate_subexpressions().into_iter().rev());
        }
        Some(result)
    }
}
