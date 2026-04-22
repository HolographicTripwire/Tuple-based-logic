use std::collections::{HashMap, HashSet};

use proof_calculus::structures::{propsets::implementations::hash::HashPropSet1O};
use tbl_proof_calculus::expressions::assigned::{TblExpression, compound::CompoundTblExpression};

use crate::expressions::assigned::{is_negation_of, negation_level};

/// Get HashMap mapping negation levels onto sets of propositions at that negation level
fn by_negation_level<'a, C: CompoundTblExpression, I: IntoIterator<Item=&'a TblExpression<C>>>(propositions: I) -> HashMap<usize,HashSet<&'a TblExpression<C>>> {
    // Create the map that will be returned
    let mut result: HashMap<usize, HashSet<&TblExpression<C>>> = HashMap::new();
    // Iterate through all propositions in this PropositionSet
    for proposition in propositions.into_iter() {
        // Get the negation level of the proposition
        let negation_level = negation_level(proposition);
        // If there is already a hashset of this negation level, add this proposition to that one
        if let Some(set) = result.get_mut(&negation_level) 
            { set.insert(proposition); }
        // Otherwise create a new hashset at that negation level
        else { 
            let mut new_set = HashSet::new();
            new_set.insert(proposition);
            result.insert(negation_level, new_set);
        }
    } 
    // Return the map
    result
}

/// Get the set of each [Proposition] within iterator which are contradicted by another [Proposition] within this iterator
pub fn get_contradictions<'a, C:CompoundTblExpression + 'a, I: IntoIterator<Item=&'a TblExpression<C>>>(propositions: I) -> HashPropSet1O<TblExpression<C>> {
    let empty_hashset = HashSet::new(); // Simple binding of a new HashSet
    // Create a set of Propositions to add to and eventually return
    let mut contradictions = HashPropSet1O::new();
    // Iterate throguh propositions by their negation level
    let by_negation_level = by_negation_level(propositions);
    for (level, set_1) in by_negation_level.iter() {
        // Get the Propositions that are at the negation level above this one
        let set_2 = by_negation_level.get(&(level+1)).unwrap_or(&empty_hashset);
        // Compare the Propositions within set and the set above this one
        for prop_1 in set_1 {
            for prop_2 in set_2 {
                if is_negation_of(&prop_2,&prop_1) { contradictions.extend([(*prop_1).clone()]) }
            }
        }
    }
    // Return the list of contradictions 
    contradictions
}

#[cfg(test)]
mod tests {
    use enum_iterator::cardinality;
    use tbl_proof_calculus::structures::{expressions::compound::r#box::BoxCompoundTblExpression, proof_calculus_derived::aliases::{propositions::TblProposition, propsets::HashTblPropSet1O}};

    use crate::structures::atoms::PhilosophicaInferenceAtoms;

    use super::*;

    #[test]
    fn test_by_negation_level_with_empty_set() {
        let propset: HashTblPropSet1O<BoxCompoundTblExpression> = HashPropSet1O::new();
        let expected = HashMap::new();
        assert_eq!(by_negation_level(propset.iter()),expected)
    }

    #[test]
    fn test_by_negation_level_with_full_set() {
        let neg: TblExpression<BoxCompoundTblExpression> = PhilosophicaInferenceAtoms::Negation.into();
        let x = TblExpression::Atomic(cardinality::<PhilosophicaInferenceAtoms>().try_into().unwrap());
        let y = TblExpression::Atomic((cardinality::<PhilosophicaInferenceAtoms>() + 1).try_into().unwrap());
        let neg_x = TblExpression::from(vec![neg.clone(), x.clone()]);
        let neg_neg_x = TblExpression::from(vec![neg.clone(), neg_x.clone()]);
        let propset = HashPropSet1O::from_iter(vec![x.clone(), y.clone(), neg_x.clone(), neg_neg_x.clone()]);
        let expected = [
            (0,HashSet::from_iter([&x,&y])),
            (1,HashSet::from_iter([&neg_x])),
            (2,HashSet::from_iter([&neg_neg_x]))
        ].iter().cloned().collect();
        assert_eq!(by_negation_level(propset.iter()),expected)
    }

    #[test]
    fn test_get_contradictions_with_no_contradictions() {
        let neg: TblExpression<BoxCompoundTblExpression> = PhilosophicaInferenceAtoms::Negation.into();
        let x = TblExpression::Atomic(cardinality::<PhilosophicaInferenceAtoms>().try_into().unwrap());
        let y = TblExpression::Atomic((cardinality::<PhilosophicaInferenceAtoms>() + 1).try_into().unwrap());
        let neg_x = TblExpression::from(vec![neg, x]);
        let propset = HashPropSet1O::from_iter(vec![neg_x, y]);
        let contradictions = HashPropSet1O::new();
        assert_eq!(get_contradictions(&propset), contradictions)
    }

    #[test]
    fn test_get_contradictions_with_contradictions() {
        let neg: TblExpression<BoxCompoundTblExpression> = PhilosophicaInferenceAtoms::Negation.into();
        let x = TblExpression::Atomic(cardinality::<PhilosophicaInferenceAtoms>().try_into().unwrap());
        let y = TblExpression::Atomic((cardinality::<PhilosophicaInferenceAtoms>() + 1).try_into().unwrap());
        let neg_x = TblExpression::from(vec![neg.clone(), x.clone()]);
        let neg_y = TblExpression::from(vec![neg.clone(), y.clone()]);
        let propset = HashPropSet1O::from_iter(vec![x.clone(), y.clone(), neg_x, neg_y]);
        let contradictions = HashPropSet1O::from_iter(vec![x, y]);
        assert_eq!(get_contradictions(&propset), contradictions)
    }

    #[test]
    fn test_get_contradictions_with_triple_contradiction() {
        let neg: TblExpression<BoxCompoundTblExpression> = PhilosophicaInferenceAtoms::Negation.into();
        let x = TblExpression::Atomic(cardinality::<PhilosophicaInferenceAtoms>().try_into().unwrap());
        let neg_x = TblExpression::from(vec![neg.clone(), x.clone()]);
        let neg2_x = TblExpression::from(vec![neg.clone(), neg_x.clone()]);
        let neg3_x = TblExpression::from(vec![neg.clone(), neg2_x.clone()]);
        let propset = HashPropSet1O::from_iter(vec![x.clone(), neg3_x.clone()]);
        let contradictions = HashPropSet1O::new();
        assert_eq!(get_contradictions(&propset), contradictions)
    }

    #[test]
    fn test_get_contradictions_with_improper_negation() {
        let neg: TblProposition<BoxCompoundTblExpression> = PhilosophicaInferenceAtoms::Negation.into();
        let x = TblExpression::Atomic(cardinality::<PhilosophicaInferenceAtoms>().try_into().unwrap());
        let y = TblExpression::Atomic((cardinality::<PhilosophicaInferenceAtoms>() + 1).try_into().unwrap());
        let neg_xy = TblExpression::from(vec![neg.clone(), x.clone(), y.clone()]);
        let propset = HashPropSet1O::from_iter(vec![x.clone(), y.clone(), neg_xy]);
        let contradictions = HashPropSet1O::new();
        assert_eq!(get_contradictions(&propset), contradictions)
    }
}
