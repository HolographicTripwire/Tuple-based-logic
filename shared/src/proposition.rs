use std::collections::HashSet;

use crate::term::Term;

#[derive(Hash,PartialEq,Eq,Debug,Clone)]
pub struct Proposition(pub Term);

pub struct PropositionSet(HashSet<Proposition>);

impl PropositionSet {
    pub fn merge(&mut self, other: &Self) {
        self.0.extend(other.0.iter().cloned());
    }

    pub fn merged(&self, other: &Self) -> Self {
        Self(HashSet::from_iter(self.0.iter().chain(other.0.iter()).cloned()))
    }

    pub fn extend(&mut self, vec: &Vec<Proposition>) {
        self.0.extend(vec.iter().cloned());
    }

    pub fn extended(&self, vec: &Vec<Proposition>) -> Self {
        Self(HashSet::from_iter(self.0.iter().chain(vec).cloned()))
    }

    pub fn subtract(&mut self, other: &PropositionSet) {
        self.0 = self.0.difference(&other.0).cloned().collect();
    }

    pub fn subtracted(&self, other: &PropositionSet) -> Self {
        Self(self.0.difference(&other.0).cloned().collect())
    }

    pub fn contains(&self, proposition: &Proposition) -> bool {
        self.0.contains(proposition)
    }

    pub fn subset_of(&self, other: &PropositionSet) -> bool {
        for proposition in &self.0 {
            if !other.contains(&proposition) { return false; }
        } return true;
    }

}

impl From<&Proposition> for PropositionSet {
    fn from(proposition: &Proposition) -> Self {
        Self(HashSet::from_iter(vec![proposition.clone()].iter().cloned()))
    }
}

impl From<&Vec<Proposition>> for PropositionSet {
    fn from(starting_propositions: &Vec<Proposition>) -> Self {
        Self(HashSet::from_iter(starting_propositions.iter().cloned()))
    }
}

impl From<&HashSet<Proposition>> for PropositionSet {
    fn from(starting_propositions: &HashSet<Proposition>) -> Self {
        Self(starting_propositions.clone())
    }
}
