use std::{collections::{HashMap, HashSet}};

use crate::expressions::Expression;

/// Every [Proposition] within Tuple-based Logic is simply an [Expression] whose truth value is to be considered
pub type Proposition = Expression;
pub type PropositionSet = HashSet<Proposition>;

/// Get HashMap mapping negation levels onto sets of propositions at that negation level
fn by_negation_level<'a, I: IntoIterator<Item=&'a Proposition>>(propositions: I) -> HashMap<usize,HashSet<&'a Proposition>> {
    // Create the map that will be returned
    let mut result: HashMap<usize, HashSet<&Proposition>> = HashMap::new();
    // Iterate through all propositions in this PropositionSet
    for proposition in propositions.into_iter() {
        // Get the negation level of the proposition
        let negation_level = proposition.negation_level();
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
pub fn get_contradictions<'a, I: IntoIterator<Item=&'a Proposition>>(propositions: I) -> PropositionSet {
    let empty_hashset = HashSet::new(); // Simple binding of a new HashSet
    // Create a set of Propositions to add to and eventually return
    let mut contradictions = PropositionSet::new();
    // Iterate throguh propositions by their negation level
    let by_negation_level = by_negation_level(propositions);
    for (level, set_1) in by_negation_level.iter() {
        // Get the Propositions that are at the negation level above this one
        let set_2 = by_negation_level.get(&(level+1)).unwrap_or(&empty_hashset);
        // Compare the Propositions within set and the set above this one
        for prop_1 in set_1 {
            for prop_2 in set_2 {
                if prop_2.is_negation_of(&prop_1) { contradictions.extend([(*prop_1).clone()]) }
            }
        }
    }
    // Return the list of contradictions 
    contradictions
}

#[cfg(test)]
mod tests {
    use enum_iterator::cardinality;

    use crate::{atoms::{BuiltInAtom}, expressions::{Expression}};

    use super::*;

    #[test]
    fn test_get_contradictions_with_no_contradictions() {
        let neg = BuiltInAtom::Negation.into();
        let x = Expression::Atomic(cardinality::<BuiltInAtom>().try_into().unwrap());
        let y = Expression::Atomic((cardinality::<BuiltInAtom>() + 1).try_into().unwrap());
        let neg_x = Expression::Tuple(vec![neg, x]);
        let propset = PropositionSet::from_iter(vec![neg_x, y]);
        let contradictions = PropositionSet::new();
        assert_eq!(get_contradictions(&propset), contradictions)
    }

    #[test]
    fn test_get_contradictions_with_contradictions() {
        let neg: Expression = BuiltInAtom::Negation.into();
        let x = Expression::Atomic(cardinality::<BuiltInAtom>().try_into().unwrap());
        let y = Expression::Atomic((cardinality::<BuiltInAtom>() + 1).try_into().unwrap());
        let neg_x = Expression::Tuple(vec![neg.clone(), x.clone()]);
        let neg_y = Expression::Tuple(vec![neg.clone(), y.clone()]);
        let propset = PropositionSet::from_iter(vec![x.clone(), y.clone(), neg_x, neg_y]);
        let contradictions = PropositionSet::from_iter(vec![x, y]);
        assert_eq!(get_contradictions(&propset), contradictions)
    }

    #[test]
    fn test_get_contradictions_with_triple_contradiction() {
        let neg: Expression = BuiltInAtom::Negation.into();
        let x = Expression::Atomic(cardinality::<BuiltInAtom>().try_into().unwrap());
        let neg_x = Expression::Tuple(vec![neg.clone(), x.clone()]);
        let neg2_x = Expression::Tuple(vec![neg.clone(), neg_x.clone()]);
        let neg3_x = Expression::Tuple(vec![neg.clone(), neg2_x.clone()]);
        let propset = PropositionSet::from_iter(vec![x.clone(), neg3_x.clone()]);
        let contradictions = PropositionSet::new();
        assert_eq!(get_contradictions(&propset), contradictions)
    }

    #[test]
    fn test_get_contradictions_with_improper_negation() {
        let neg: Expression = BuiltInAtom::Negation.into();
        let x = Expression::Atomic(cardinality::<BuiltInAtom>().try_into().unwrap());
        let y = Expression::Atomic((cardinality::<BuiltInAtom>() + 1).try_into().unwrap());
        let neg_xy = Expression::Tuple(vec![neg.clone(), x.clone(), y.clone()]);
        let propset = PropositionSet::from_iter(vec![x.clone(), y.clone(), neg_xy]);
        let contradictions = PropositionSet::new();
        assert_eq!(get_contradictions(&propset), contradictions)
    }
}
