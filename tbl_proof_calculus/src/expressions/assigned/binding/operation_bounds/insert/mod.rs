use proof_calculus::{propositions::assigned::binding::bounds::InsertBoundsForProp, utils::collections::{binders::InsertBinder, multimap::MultiMap}};

use crate::expressions::assigned::{TblExpression, at_path_enum::TblExpressionAtPathEnum, binding::bounds::{TblExpressionBoundAtomExactValue, TblExpressionBoundCompoundExactLength, TblExpressionBoundValueDuplicated, TblExpressionInsertionBound}, compound::CompoundTblExpression, subexpressions::iterators::depth_first::counterclockwise::CounterclockwiseDepthFirstLocatedTblSubexpressionIterator};

#[derive(Clone,PartialEq,Eq,Hash,Debug)]
pub struct TblFastConstructInsertionBoundsForExpr(Box<[TblExpressionInsertionBound]>);
pub type TblFastConstructInsertionBoundsForProp = TblFastConstructInsertionBoundsForExpr;
impl <'prop,C: 'prop + CompoundTblExpression,B:InsertBinder<Self>> InsertBoundsForProp<'prop,TblExpression<C>,B> for TblFastConstructInsertionBoundsForProp {}

impl <'a, C: CompoundTblExpression> From<&'a TblExpression<C>> for TblFastConstructInsertionBoundsForExpr {
    fn from(expr: &'a TblExpression<C>) -> Self {
        // Initialise dups
        let mut dup_atoms = MultiMap::new();
        let mut dup_compounds = MultiMap::new();
        // Construct element bounds
        let mut bounds: Vec<_> = CounterclockwiseDepthFirstLocatedTblSubexpressionIterator::new(expr)
            .map(|v| { 
                match v.into() {
                    TblExpressionAtPathEnum::Atomic(atom) => {
                        dup_atoms.insert(atom.obj, atom.path.clone());
                        TblExpressionBoundAtomExactValue::new(atom.path, *atom.obj).into()
                    },
                    TblExpressionAtPathEnum::Compound(compound) => {
                        dup_compounds.insert(compound.obj.clone(), compound.path.clone());
                        TblExpressionBoundCompoundExactLength::new(compound.path, compound.obj.len()).into()
                    }
                }
            }).collect();
        // Construct duplicate bounds
        let dups = dup_atoms
            .into_values()
            .into_iter()
            .chain(dup_compounds.into_values());
        for values in dups {
            let values: Vec<_> = values.into_iter().collect();
            for i in 0..values.len() {
                let ix = &values[i];
                for j in i+1..values.len() {
                    bounds.push(TblExpressionBoundValueDuplicated::new(ix.clone(), values[j].clone()).0.into())
                }
            }
        }
        // Combine the elements
        Self(bounds.into())
    }
}

impl TblFastConstructInsertionBoundsForExpr {
    pub fn bounds(&self) -> &Box<[TblExpressionInsertionBound]> { &self.0 }
}
