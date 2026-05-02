use proof_calculus::{propositions::types::unassigned::binding::bounds::InsertBoundsForUprop, utils::collections::{binding::binders::InsertBinder, maps::multimap::MultiMap}};

use crate::expressions::types::{assigned::binding::bounds::{TblExpressionBoundAtomExactValue, TblExpressionBoundCompoundExactLength, TblExpressionBoundValueDuplicated}, unassigned::{UnassignedTblExpression, at_path_enum::UnassignedTblExpressionAtPathEnum, binding::bounds::{UnassignedTblExpressionBoundVariableExactValue, UnassignedTblExpressionInsertionBound}, compound::UnassignedCompoundTblExpression, subexpressions::iterators::{depth_first::counterclockwise::CounterclockwiseDepthFirstLocatedUnassignedTblSubexpressionIterator}}};

#[derive(Clone,PartialEq,Eq,Hash,Debug)]
pub struct TblFastConstructInsertionBoundsForUexpr(Box<[UnassignedTblExpressionInsertionBound]>);
pub type TblFastConstructInsertionBoundsForUprop = TblFastConstructInsertionBoundsForUexpr;
impl <'prop,C: 'prop + UnassignedCompoundTblExpression,B:InsertBinder<Self>> InsertBoundsForUprop<'prop,UnassignedTblExpression<C>,B> for TblFastConstructInsertionBoundsForUprop {}

impl <'a, C: UnassignedCompoundTblExpression> From<&'a UnassignedTblExpression<C>> for TblFastConstructInsertionBoundsForUexpr {
    fn from(expr: &'a UnassignedTblExpression<C>) -> Self {
        // Initialise dups
        let mut dup_atoms = MultiMap::new();
        let mut dup_variables = MultiMap::new();
        let mut dup_compounds = MultiMap::new();
        // Construct element bounds
        let mut bounds: Vec<_> = CounterclockwiseDepthFirstLocatedUnassignedTblSubexpressionIterator::new(expr)
            .map(|v| { 
                match v.into() {
                    UnassignedTblExpressionAtPathEnum::Atomic(atom) => {
                        dup_atoms.insert(atom.obj, atom.path.clone());
                        TblExpressionBoundAtomExactValue::new(atom.path, *atom.obj).into()
                    },
                    UnassignedTblExpressionAtPathEnum::Variable(variable) => {
                        dup_variables.insert(variable.obj, variable.path.clone());
                        UnassignedTblExpressionBoundVariableExactValue::new(variable.path, *variable.obj).into()
                    },
                    UnassignedTblExpressionAtPathEnum::Compound(compound) => {
                        dup_compounds.insert(compound.obj.clone(), compound.path.clone());
                        TblExpressionBoundCompoundExactLength::new(compound.path, compound.obj.len()).into()
                    }
                }
            }).collect();
        // Construct duplicate bounds
        let dups = dup_atoms
            .into_values()
            .into_iter()
            .chain(dup_variables.into_values())
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

impl TblFastConstructInsertionBoundsForUexpr {
    pub fn bounds(&self) -> &Box<[UnassignedTblExpressionInsertionBound]> { &self.0 }
}
