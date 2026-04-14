use std::collections::HashSet;

use itertools::Itertools;
use proof_calculus::structures::propositions::bounds::PropositionIdentityBounds;

use crate::structures::expressions::{TblExpression, bounds::{TblExpressionIdentityBound, iterators::fast_construct::TblExpressionFastConstructIdentityBounds}, compound::CompoundTblExpression, subexpressions::TblSubexpressionInExpressionPath};

#[derive(Clone,Copy,PartialEq,Eq,Hash,Debug)]
pub struct PathEliminationHeuristic {
    pub atom_multiplier: u8,
    pub compound_length_threshold: u8,
}
impl PathEliminationHeuristic {
    fn evaluate(&self, bound: &TblExpressionIdentityBound, important_paths: &HashSet<TblSubexpressionInExpressionPath>) -> isize { match bound {
        TblExpressionIdentityBound::AtomValue(atom_bound) =>
            { if important_paths.contains(&atom_bound.path) { Self::depth_value(&atom_bound.path) } else { -1 } },
        TblExpressionIdentityBound::CompoundLength(compound_bound) => {
            if compound_bound.length == 0 && important_paths.contains(&compound_bound.path) { Self::depth_value(&compound_bound.path) }
            else { let v: isize = compound_bound.length.try_into().unwrap(); -1-v }
        },
    }}
    fn depth_value(path: &TblSubexpressionInExpressionPath) -> isize {
        path.0.iter()
            .map(|v| v.0)
            .sum::<usize>()
            .into()
            + path.0.len()
    }
}
impl Default for PathEliminationHeuristic {
    fn default() -> Self { Self {
        atom_multiplier: 4, 
        compound_length_threshold: 3
    }}
}

pub struct TblExpressionFastLookupIdentityBounds(Vec<TblExpressionIdentityBound>);

impl TblExpressionFastLookupIdentityBounds {
    pub fn new<C: CompoundTblExpression>(expr: &TblExpression<C>, heuristic: PathEliminationHeuristic) -> Self {
        let (vec, important_paths) = TblExpressionFastConstructIdentityBounds::new(expr).bounds_and_important();
        Self(vec.into_iter().sorted_by_cached_key(
            |bound| heuristic.evaluate(&bound, &important_paths)
        ).collect())
    }
}
