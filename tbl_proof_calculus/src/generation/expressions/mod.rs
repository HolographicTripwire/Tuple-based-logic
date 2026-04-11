use std::collections::HashMap;

use proof_calculus::utils::collections::dense_usize_map::DenseUsizeMap;

use crate::{generation::expressions::compound::UnassignedCompoundTblExpression, structures::expressions::{TblExpression, atomic::AtomicTblExpression, compound::CompoundTblExpression, subexpressions::TblSubexpressionInExpressionPath}};

pub mod compound;
pub mod subexpressions;

#[derive(Clone,PartialEq,Eq,Hash,Debug)]
pub enum UnassignedTblExpression<C: UnassignedCompoundTblExpression> {
    Atomic(AtomicTblExpression),
    Compound(C),
    Variable(usize)
}
impl <C: UnassignedCompoundTblExpression> UnassignedTblExpression<C> {
    pub fn replace(&self, to_replace: &UnassignedTblExpression<C>, replace_with: &UnassignedTblExpression<C>) -> Self {
        if self == to_replace { replace_with.clone() }
        else if let UnassignedTblExpression::Compound(compound) = self
            { UnassignedTblExpression::Compound(compound.replace(to_replace, replace_with)) }
        else { self.clone() }
    }

    pub fn is_atom(&self) -> bool { if let UnassignedTblExpression::Atomic(_) = self { true } else { false } }
    pub fn is_compound(&self) -> bool { if let UnassignedTblExpression::Compound(_) = self { true } else { false } }
    pub fn is_variable(&self) -> bool { if let UnassignedTblExpression::Variable(_) = self { true } else { false } }

    // pub fn get_subexpressions_helper(&self,path: &TblSubexpressionInExpressionPath, index: usize) -> Result<&UnassignedTblExpression<C>,()> {
    //     let immediate_path = path.0.get(index).ok_or(())?;
    //     let inner = self.get_immediate_subexpression(immediate_path)?;
    //     if index == path.0.len() { Ok(inner) }
    //     else { inner.get_subexpressions_helper(path, index+1) }
    // }

    // /// If this expression is a Tuple, get its expressions. Otherwise throw an error 
    // pub fn as_vec<'a>(&'a self) -> Result<&'a C,()> { 
    //     match self {
    //         TblExpression::Atomic(_) => Err(()),
    //         TblExpression::Compound(proposition_exprs) => Ok(proposition_exprs),
    //     }
    // }

    /// If this expression is a Tuple, get its subexpressions. Otherwise throw an error 
    pub fn as_slice(&self) -> Result<&[UnassignedTblExpression<C>], ()> {
        match self {
            UnassignedTblExpression::Compound(proposition_exprs) => Ok(proposition_exprs.as_slice()),
            _ => Err(()),
        }
    }

    pub fn len(&self) -> Option<usize> {
        match self {
            UnassignedTblExpression::Compound(exprs) => Some(exprs.len()),
            _ => None,
        }
    }
}
impl <C: UnassignedCompoundTblExpression> From<TblExpression<C::InnerCompound>> for UnassignedTblExpression<C> {
    fn from(value: TblExpression<C::InnerCompound>) -> Self { match value {
        TblExpression::Atomic(atom) => UnassignedTblExpression::Atomic(atom),
        TblExpression::Compound(compound) => UnassignedTblExpression::Compound(compound.into()),
    }}
}

#[derive(Clone,PartialEq,Eq,Debug)]
pub struct TblExpressionAssignment<C: CompoundTblExpression>(pub DenseUsizeMap<TblExpression<C>>);
impl <C: CompoundTblExpression> Default for TblExpressionAssignment<C> {
    fn default() -> Self { Self(Default::default()) }
}
impl <const N: usize, C:CompoundTblExpression> From<[(usize,TblExpression<C>); N]> for TblExpressionAssignment<C> {
    fn from(pairs: [(usize,TblExpression<C>); N]) -> Self { Self(pairs.into()) }
}
impl <C: CompoundTblExpression> From<Vec<(usize,TblExpression<C>)>> for TblExpressionAssignment<C> {
    fn from(pairs: Vec<(usize,TblExpression<C>)>) -> Self { Self(pairs.into()) }
}

#[derive(Clone,PartialEq,Eq,Debug)]
pub struct PartialTblExpressionAssignment<C: UnassignedCompoundTblExpression>(pub DenseUsizeMap<UnassignedTblExpression<C>>);
impl <C: UnassignedCompoundTblExpression> Default for PartialTblExpressionAssignment<C> {
    fn default() -> Self { Self(Default::default()) }
}
impl <const N: usize, C: UnassignedCompoundTblExpression> From<[(usize,UnassignedTblExpression<C>); N]> for PartialTblExpressionAssignment<C> {
    fn from(pairs: [(usize,UnassignedTblExpression<C>); N]) -> Self { Self(pairs.into()) }
}
impl <C: UnassignedCompoundTblExpression> From<Vec<(usize,UnassignedTblExpression<C>)>> for PartialTblExpressionAssignment<C> {
    fn from(pairs: Vec<(usize,UnassignedTblExpression<C>)>) -> Self { Self(pairs.into()) }
}
