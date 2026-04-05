use std::collections::HashMap;

use crate::{generation::expressions::compound::UnassignedCompoundTblExpression, structures::expressions::{TblExpression, atomic::AtomicTblExpression, compound::CompoundTblExpression}};

pub mod compound;
pub mod subexpressions;

#[derive(Clone,PartialEq,Eq,Hash,Debug)]
pub enum UnassignedTblExpression<C: UnassignedCompoundTblExpression> {
    Atom(AtomicTblExpression),
    Compound(C),
    Variable(usize)
}

#[derive(Clone,PartialEq,Eq,Debug)]
pub struct TblExpressionAssignment<C: CompoundTblExpression> {
    inner: HashMap<usize,TblExpression<C>>
}
#[derive(Clone,PartialEq,Eq,Debug)]
pub struct PartialTblExpressionAssignment<C: UnassignedCompoundTblExpression> {
    inner: HashMap<usize,UnassignedTblExpression<C>>
}
