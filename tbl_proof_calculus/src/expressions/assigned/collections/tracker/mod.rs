
use std::{collections::{HashMap, HashSet}, hash::Hash};

use proof_calculus::{propositions::collections::binders::{GetBinderForPropIdenticalToProp, InsertBinderForProp, unassigned::GetBinderForPropsSubsumedByUprop}, utils::collections::binders::{Binder, GetBinder}};

use crate::{expressions::{assigned::{binding::{TblPropositionIdentityBound, atom::TblPropositionBoundAtomExactValue, compound::TblPropositionBoundCompoundExactLength, duplication::TblPropositionBoundValueDuplicated}, collections::tracker::bounds::{TblExpressionTrackerBoundsAtomExactValue, TblExpressionTrackerCompoundLengthBounds, TblExpressionTrackerDuplicationBounds}, compound::CompoundTblExpression, subexpressions::TblSubexpressionInExpressionPath}, unassigned::compound::UnassignedCompoundTblExpression}, proof_calculus_derived::aliases::propositions::{TblProposition, UnassignedTblProposition}};

pub mod bounds;

pub struct TblExpressionBinder<T: Hash + Eq + Clone> {
    atom_value_bounds: TblExpressionTrackerBoundsAtomExactValue<T>,
    compound_length_bounds: TblExpressionTrackerCompoundLengthBounds<T>,
    duplicate_value_bounds: TblExpressionTrackerDuplicationBounds<T>,
}
impl <T: Hash + Eq + Clone> TblExpressionBinder<T> {
    fn get_unbounded_path_matches(&self, path: &TblSubexpressionInExpressionPath) -> HashSet<&T> {
        match (self.atom_value_bounds.get_no_bound(path), self.compound_length_bounds.get_no_bound(path)) {
            (None, None) => HashSet::new(),
            (None, Some(v)) => v.into_iter().collect(),
            (Some(v), None) => v.into_iter().collect(),
            (Some(v1), Some(v2)) => v1.into_iter().chain(v2.into_iter()).collect(),
        }
    }
}

pub type TblPropositionTracker<T> = TblExpressionBinder<T>;
impl <T: Hash + Eq + Clone> Binder for TblPropositionTracker<T> {
    type Value = T;
    
    #[inline]
    fn get_all<'a>(&'a self) -> HashSet<&'a Self::Value> { self.get_unbounded_path_matches(&TblSubexpressionInExpressionPath::default()) }
}
impl <T: Hash + Eq + Clone> GetBinder<TblPropositionBoundAtomExactValue> for TblPropositionTracker<T> {
    #[inline]
    fn get<'a>(&'a self, key: &TblPropositionBoundAtomExactValue) -> HashSet<&'a Self::Value>
        { self.atom_value_bounds.get2(&key) }
}
impl <T: Hash + Eq + Clone> GetBinder<TblPropositionBoundCompoundExactLength> for TblPropositionTracker<T> {
    #[inline]
    fn get<'a>(&'a self, key: &TblPropositionBoundCompoundExactLength) -> HashSet<&'a Self::Value>
        { self.compound_length_bounds.get2(&key) }
}
impl <T: Hash + Eq + Clone> GetBinder<TblPropositionBoundValueDuplicated> for TblPropositionTracker<T> {
    #[inline]
    fn get<'a>(&'a self, key: &TblPropositionBoundValueDuplicated) -> HashSet<&'a Self::Value>
        { self.duplicate_value_bounds.get2(&key) }
}
impl <T: Hash + Eq + Clone> GetBinder<TblPropositionIdentityBound> for TblPropositionTracker<T> {
    #[inline]
    fn get<'a>(&'a self, key: &TblPropositionIdentityBound) -> HashSet<&'a Self::Value> { match key {
        crate::expressions::assigned::binding::TblExpressionIdentityBound::AtomValue(atom_bound) => self.get(atom_bound),
        crate::expressions::assigned::binding::TblExpressionIdentityBound::CompoundLength(compound_bound) => self.get(compound_bound),
    }}
}


impl <C: CompoundTblExpression, T: Hash + Eq + Clone> GetBinderForPropIdenticalToProp<TblProposition<C>> for TblPropositionTracker<T> {
    type DefaultGetBoundsForPropIdenticalToProp<'a> = Fast where TblProposition<C>: 'a;
}
impl <C: UnassignedCompoundTblExpression, T: Hash + Eq + Clone> GetBinderForPropsSubsumedByUprop<UnassignedTblProposition<C>> for TblPropositionTracker<T> {
    type DefaultGetBoundsForPropsSubsumedByUprop<'a> = usize where TblProposition<C>: 'a;
}

impl <'a, C: CompoundTblExpression, T: Hash + Eq + Clone> InsertBinderForProp<'a, TblProposition<C>> for TblPropositionTracker<T> {
    type DefaultInsertionBounds = usize where TblProposition<C>: 'a;
    fn insert_prop<Bs: proof_calculus::propositions::bounds::InsertBoundsForProp<'a,TblProposition<C>,Self,Self::Value>>(&'a mut self, bounds: Bs, value: Self::Value) where TblProposition<C>: 'a {
        todo!()
    }
}

// pub type TblPropositionTracker<T: Hash + Eq + Clone> = TblExpressionTracker<T>;
// impl <C: CompoundTblExpression, T: Hash + Eq + Clone> BindGetPropositionsByBounds<TblProposition<C>> for TblPropositionTracker<T> {
//     type Value = T;

//     fn insert(&mut self, key: TblProposition<C>, value: Self::Value) {
//         todo!()
//     }
// }


// impl <T: Hash + Eq + Clone> TblExpressionTracker<T> {
//     pub fn get_identical<'a, C: CompoundTblExpression>(&'a self, find: &TblExpression<C>) -> impl IntoIterator<Item=&'a T> {
//         match find {
//             TblExpression::Atomic(atom) => self.atom_value_bounds.get(&TblSubexpressionInExpressionPath::default(), atom),
//             TblExpression::Compound(compound) => {
//                 let mut found = self.compound_length_bounds.get(&TblSubexpressionInExpressionPath::default(), compound.len());
//                 for expr in compound.get_located_immediate_subexpressions() {
//                     if found.len() == 0 { return found }
//                     found = self.get_identical_helper(expr.transform_path(), found)
//                 }
//                 found
//             },
//         }
//     }
//     fn get_identical_helper<'a, C: CompoundTblExpression>(&'a self, find: TblSubexpressionInExpression<C>, mut found: HashSet<&'a T>) -> HashSet<&'a T> {
//         match find.into() {
//             TblExpressionAtPathEnum::Atomic(atom) => {
//                 let atom_bound_matches = self.atom_value_bounds.get(&atom.path, atom.obj);
//                 found.intersection(&atom_bound_matches).map(|v| *v).collect()
//             }, TblExpressionAtPathEnum::Compound(compound) => {
//                 let compound_bound_matches = self.compound_length_bounds.get(&compound.path,compound.obj.len());
//                 found = found.intersection(&compound_bound_matches).map(|v| *v).collect();
//                 for expr in compound.into_located_immediate_subexpressions() {
//                     if found.len() == 0 { break } // No point taking further intersections on an empty set
//                     found = self.get_identical_helper(expr, found)
//                 }
//                 found
//             },
//         }
//     }

//     pub fn get_subsumed_by<'a, C: UnassignedCompoundTblExpression>(&'a self, find: &UnassignedTblExpression<C>) -> impl IntoIterator<Item=&'a T> {
//         match find {
//             UnassignedTblExpression::Atomic(atom) => self.atom_value_bounds.get(&TblSubexpressionInExpressionPath::default(), atom),
//             UnassignedTblExpression::Compound(compound) => {
//                 let mut found = self.compound_length_bounds.get(&TblSubexpressionInExpressionPath::default(), compound.len());
//                 for expr in compound.get_located_immediate_subexpressions() {
//                     if found.len() == 0 { return found }
//                     found = self.get_subsumed_by_helper(expr.transform_path(), found, &mut HashMap::new())
//                 }
//                 found
//             }, UnassignedTblExpression::Variable(_) =>  self.get_unbounded_path_matches(&TblSubexpressionInExpressionPath::default()),
//         }
//     }
//     fn get_subsumed_by_helper<'a, C: UnassignedCompoundTblExpression>(&'a self, find: UnassignedTblSubexpressionInExpression<C>, mut found: HashSet<&'a T>, earliest_var_paths: &mut HashMap<TblExpressionVariable,TblSubexpressionInExpressionPath>) -> HashSet<&'a T> {
//         match find.into() {
//             UnassignedTblSubexpressionInExpressionEnum::Atomic(atom) => {
//                 let atom_bound_matches = self.atom_value_bounds.get(&atom.path, atom.obj);
//                 found.intersection(&atom_bound_matches).map(|v| *v).collect()
//             }, UnassignedTblSubexpressionInExpressionEnum::Compound(compound) => {
//                 let compound_bound_matches = self.compound_length_bounds.get(&compound.path,compound.obj.len());
//                 found = found.intersection(&compound_bound_matches).map(|v| *v).collect();
//                 for expr in compound.into_located_immediate_subexpressions() {
//                     if found.len() == 0 { break } // No point taking further intersections on an empty set
//                     found = self.get_subsumed_by_helper(expr, found, earliest_var_paths)
//                 }
//                 found
//             }, UnassignedTblSubexpressionInExpressionEnum::Variable(variable) => {
//                 match earliest_var_paths.get(variable.obj) {
//                     Some(path) => {
//                         let duplicate_bound = self.duplicate_value_bounds.get(path, &variable.path);
//                         found = found.intersection(&duplicate_bound).map(|v| *v).collect();
//                     },
//                     None => { earliest_var_paths.insert(*variable.obj, variable.path); },
//                 };
//                 found
//             },
//         }
//     }
    
//     pub fn insert<C: CompoundTblExpression>(&mut self, key: &TblExpression<C>, value: &T) {
//         let mut duplicates = MultiMap::new();
//         self.insert_helper(
//             TblSubexpressionInExpression{ obj: &key, path: TblSubexpressionInExpressionPath::default()},
//             &mut duplicates,
//             value.clone()
//         );
//         self.insert_dups_helper(duplicates, value);
//     }
//     fn insert_helper<'a, C: CompoundTblExpression>(&mut self, expr: TblSubexpressionInExpression<'a,C>, duplicates: &mut MultiMap<RcTblExpression,TblSubexpressionInExpressionPath>, value: T) -> RcTblExpression {
//         //let expr_rc = expr.obj.clone();
//         let path = expr.path.clone();
//         let obj = match expr.into() {
//             TblSubexpressionInExpressionEnum::Atomic(atomic) => {
//                 self.atom_value_bounds.insert(&atomic.path, atomic.obj.clone(), value);
//                 TblExpression::Atomic(*atomic.obj)
//             },
//             TblSubexpressionInExpressionEnum::Compound(compound) => {
//                 self.compound_length_bounds.insert(&compound.path, compound.obj.len(), value.clone());
//                 let elements = compound
//                     .into_located_subexpressions()
//                     .into_iter()
//                     .map(|v | self.insert_helper(v, duplicates, value.clone()));
//                 TblExpression::Compound(RcCompoundTblExpression::from_iter(elements))
//             },
//         };
//         duplicates.insert(obj.clone(),path);
//         return obj
//     }
//     fn insert_dups_helper(&mut self, map: MultiMap<RcTblExpression,TblSubexpressionInExpressionPath>, value: &T) {
//         for values in map.into_values() {
//             let values: Vec<_> = values.into_iter().collect();
//             for i in 0..values.len() {
//                 let ix = &values[i];
//                 for j in i+1..values.len() {
//                     self.duplicate_value_bounds.insert(ix.clone(), values[j].clone(), value.clone());
//                 }
//             }
//         }
//     }
// }
