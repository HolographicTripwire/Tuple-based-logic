use std::{collections::{HashMap, HashSet}};

use super::Proposition;

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct PropositionSet(HashSet<Proposition>);

impl PropositionSet {
    /// Create a new [`PropositionSet`] with some set of starting [Proposition] objects
    pub fn new<'a>(starting_propositions: impl IntoIterator<Item = &'a Proposition>) -> Self { Self(starting_propositions.into_iter().cloned().collect()) }

    /// Add every [`Proposition`] in a provided iterator of [Proposition] to this [PropositionSet]
    pub fn merge<'a>(&mut self, iter: impl IntoIterator<Item = &'a Proposition>) { self.0.extend(iter.into_iter().cloned()); }
    /// Get the [`PropositionSet`] that would result from adding every [`Proposition`] in a provided [Iterator] of [Proposition] objects to this [PropositionSet]
    pub fn merged<'a>(&self, iter: impl IntoIterator<Item = &'a Proposition>) -> Self {
        let iter2: Vec<&Proposition> = iter.into_iter().collect();
        Self(self.0.iter().chain(iter2.iter().cloned()).cloned().collect())
    }

    /// Add every [`Proposition`] in another [`PropositionSet`] from this one
    pub fn subtract(&mut self, other: &PropositionSet) { self.0 = self.0.difference(&other.0).cloned().collect(); }
    /// Get the [`PropositionSet`] that would result from subtracting every [`Proposition`] in another [`PropositionSet`] from this one
    pub fn subtracted(&self, other: &PropositionSet) -> Self { Self(self.0.difference(&other.0).cloned().collect()) }

    /// Check if this [`PropositionSet`] contains the provided [`Proposition`]
    pub fn contains(&self, proposition: &Proposition) -> bool { self.0.contains(proposition) }
    /// Check if this [`PropositionSet`] contains every provided [`Proposition`]
    pub fn contains_all<'a>(&self, propositions: impl IntoIterator<Item=&'a Proposition>) -> bool {
        for proposition in propositions { if !self.contains(proposition) { return false; } }
        true
    }

    /// Check if this [`PropositionSet`] contains every [`Proposition`] in another provided [`PropositionSet`]
    pub fn subset_of(&self, other: &PropositionSet) -> bool { self.0.is_subset(&other.0) }

    /// Get the number of [`Proposition`] objects that this [`PropositionSet`] contains
    pub fn len(&self) -> usize { self.0.len() }
    /// Check if this [`PropositionSet`] contains no [`Proposition`] objects
    pub fn is_empty(&self) -> bool { self.0.is_empty() }

    /// Get HashMap mapping negation levels onto sets of propositions at that negation level
    fn by_negation_level(&self) -> HashMap<usize,HashSet<&Proposition>> {
        // Create the map that will be returned
        let mut result: HashMap<usize, HashSet<&Proposition>> = HashMap::new();
        // Iterate through all propositions in this PropositionSet
        for proposition in self.0.iter() {
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

    /// Get the set of each [Proposition] within this [PropositionSet] which are contradicted by another [Proposition] within this [PropositionSet]
    pub fn get_contradictions(&self) -> PropositionSet {
        let empty_hashset = HashSet::new(); // Simple binding of a new HashSet
        // Create a set of Propositions to add to and eventually return
        let mut contradictions = PropositionSet::new(&[]);
        // Iterate throguh propositions by their negation level
        let by_negation_level = self.by_negation_level();
        for (level, set_1) in by_negation_level.iter() {
            // Get the Propositions that are at the negation level above this one
            let set_2 = by_negation_level.get(&(level+1)).unwrap_or(&empty_hashset);
            // Compare the Propositions within set and the set above this one
            for prop_1 in set_1 {
                for prop_2 in set_2 {
                    if prop_2.is_negation_of(&prop_1) { contradictions.merge([(*prop_1)]) }
                }
            }
        }
        // Return the list of contradictions 
        contradictions
    }
}

impl <'a> IntoIterator for &'a PropositionSet {
    type IntoIter = std::collections::hash_set::Iter<'a,Proposition>;
    type Item = &'a Proposition;
    
    fn into_iter(self) -> Self::IntoIter
        { self.0.iter() }
}

impl From<&Proposition> for PropositionSet {
    fn from(proposition: &Proposition) -> Self
        { Self([proposition.clone()].iter().cloned().collect()) }
}
impl From<&Vec<Proposition>> for PropositionSet {
    fn from(starting_propositions: &Vec<Proposition>) -> Self
        { Self(starting_propositions.iter().cloned().collect()) }
}
impl From<&HashSet<Proposition>> for PropositionSet {
    fn from(starting_propositions: &HashSet<Proposition>) -> Self 
        { Self(starting_propositions.clone()) }
}

#[cfg(test)]
mod tests {
    use enum_iterator::cardinality;

    use crate::{atoms::{AtomId, BuiltInAtom}, propositions::{Expression}};

    use super::*;

    fn atomic_proposition_vec(nums: Vec<usize>) -> Vec<Proposition> {
        nums.iter()
            .map(|num| -> Proposition {
                Proposition::Atomic(AtomId::try_from(*num).unwrap())
            }).collect()
    }

    fn atomic_proposition_set(nums: Vec<usize>) -> PropositionSet {
        PropositionSet::new(atomic_proposition_vec(nums).as_slice())
    }

    #[test]
    fn test_merged() {
        let propset_01 = atomic_proposition_set(vec![0,1]);
        let propvec_12 = atomic_proposition_vec(vec![1,2]);
        let propset_012 = atomic_proposition_set(vec![0,1,2]);
        assert_eq!(propset_01.merged(&propvec_12), propset_012);
    }

    #[test]
    fn test_merge() {
        let mut propset_01 = atomic_proposition_set(vec![0,1]);
        let propvec_12 = atomic_proposition_vec(vec![1,2]);
        let propset_012 = atomic_proposition_set(vec![0,1,2]);
        propset_01.merge(&propvec_12);
        assert_eq!(propset_01, propset_012);
    }

    #[test]
    fn test_merged_with_other_propset() {
        let propset_01 = atomic_proposition_set(vec![0,1]);
        let propset_12 = atomic_proposition_set(vec![1,2]);
        let propset_012 = atomic_proposition_set(vec![0,1,2]);
        assert_eq!(propset_01.merged(&propset_12), propset_012);
    }

    #[test]
    fn test_merge_with_other_propset() {
        let mut propset_01 = atomic_proposition_set(vec![0,1]);
        let propset_12 = atomic_proposition_set(vec![1,2]);
        let propset_012 = atomic_proposition_set(vec![0,1,2]);
        propset_01.merge(&propset_12);
        assert_eq!(propset_01, propset_012);
    }

    #[test]
    fn test_subtracted() {
        let propset_01 = atomic_proposition_set(vec![0,1]);
        let propset_12 = atomic_proposition_set(vec![1,2]);
        let propset_0 = atomic_proposition_set(vec![0]);
        assert_eq!(propset_01.subtracted(&propset_12), propset_0);
    }

    #[test]
    fn test_subtract() {
        let mut propset_01 = atomic_proposition_set(vec![0,1]);
        let propset_12 = atomic_proposition_set(vec![1,2]);
        let propset_0 = atomic_proposition_set(vec![0]);
        propset_01.subtract(&propset_12);
        assert_eq!(propset_01, propset_0);
    }

    #[test]
    fn test_len() {
        for i in 0..10 {
            let propset = atomic_proposition_set((0..i).collect());
            assert_eq!(propset.len(), i)
        }
    }

    #[test]
    fn test_isempty_on_empty_set() {
        let propset_empty = atomic_proposition_set(vec![]);
        assert!(propset_empty.is_empty())
    }

    #[test]
    fn test_isempty_on_nonempty_set() {
        for i in 1..10 {
            let propset = atomic_proposition_set((0..i).collect());
            assert!(!propset.is_empty())
        }
    }

    #[test]
    fn test_get_contradictions_with_no_contradictions() {
        let neg = BuiltInAtom::Negation.into();
        let x = Expression::Atomic(cardinality::<BuiltInAtom>().try_into().unwrap());
        let y = Expression::Atomic((cardinality::<BuiltInAtom>() + 1).try_into().unwrap());
        let neg_x = Expression::Tuple(vec![neg, x]);
        let propset = PropositionSet::new(vec![&neg_x, &y]);
        let contradictions = PropositionSet::new(vec![]);
        assert_eq!(propset.get_contradictions(), contradictions)
    }

    #[test]
    fn test_get_contradictions_with_contradictions() {
        let neg: Expression = BuiltInAtom::Negation.into();
        let x = Expression::Atomic(cardinality::<BuiltInAtom>().try_into().unwrap());
        let y = Expression::Atomic((cardinality::<BuiltInAtom>() + 1).try_into().unwrap());
        let neg_x = Expression::Tuple(vec![neg.clone(), x.clone()]);
        let neg_y = Expression::Tuple(vec![neg.clone(), y.clone()]);
        let propset = PropositionSet::new(vec![&x.clone(), &y.clone(), &neg_x, &neg_y]);
        let contradictions = PropositionSet::new(vec![&x, &y]);
        assert_eq!(propset.get_contradictions(), contradictions)
    }

    #[test]
    fn test_get_contradictions_with_triple_contradiction() {
        let neg: Expression = BuiltInAtom::Negation.into();
        let x = Expression::Atomic(cardinality::<BuiltInAtom>().try_into().unwrap());
        let neg_x = Expression::Tuple(vec![neg.clone(), x.clone()]);
        let neg2_x = Expression::Tuple(vec![neg.clone(), neg_x.clone()]);
        let neg3_x = Expression::Tuple(vec![neg.clone(), neg2_x.clone()]);
        let propset = PropositionSet::new(vec![&x.clone(), &neg3_x.clone()]);
        let contradictions = PropositionSet::new(vec![]);
        assert_eq!(propset.get_contradictions(), contradictions)
    }

    #[test]
    fn test_get_contradictions_with_improper_negation() {
        let neg: Expression = BuiltInAtom::Negation.into();
        let x = Expression::Atomic(cardinality::<BuiltInAtom>().try_into().unwrap());
        let y = Expression::Atomic((cardinality::<BuiltInAtom>() + 1).try_into().unwrap());
        let neg_xy = Expression::Tuple(vec![neg.clone(), x.clone(), y.clone()]);
        let propset = PropositionSet::new(vec![&x.clone(), &y.clone(), &neg_xy]);
        let contradictions = PropositionSet::new(vec![]);
        assert_eq!(propset.get_contradictions(), contradictions)
    }
}
