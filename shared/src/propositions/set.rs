use std::collections::HashSet;

use super::Proposition;

#[derive(Clone)]
pub struct PropositionSet(HashSet<Proposition>);

impl PropositionSet {
    /// Create a new [`PropositionSet`] with some set of starting [Proposition] objects
    pub fn new(starting_propositions: &[Proposition]) -> Self { Self(starting_propositions.iter().cloned().collect()) }

    /// Add every [`Proposition`] in another [`PropositionSet`] to this one
    pub fn merge(&mut self, other: &Self) { self.0.extend(other.0.iter().cloned()); }
    /// Get the [`PropositionSet`] that would result from adding every [`Proposition`] in another [`PropositionSet`] to this one
    pub fn merged(&self, other: &Self) -> Self { Self(self.0.iter().chain(other.0.iter()).cloned().collect()) }

    /// Add every [`Proposition`] in a provided slice to this [`PropositionSet`]
    pub fn extend(&mut self, vec: &[Proposition]) { self.0.extend(vec.iter().cloned()); }
    /// Get the [`PropositionSet`] that would result from adding every [`Proposition`] in a provided slice to this one
    pub fn extended(&self, vec: &[Proposition]) -> Self { Self(self.0.iter().chain(vec).cloned().collect()) }

    /// Add every [`Proposition`] in another [`PropositionSet`] from this one
    pub fn subtract(&mut self, other: &PropositionSet) { self.0 = self.0.difference(&other.0).cloned().collect(); }
    /// Get the [`PropositionSet`] that would result from subtracting every [`Proposition`] in another [`PropositionSet`] from this one
    pub fn subtracted(&self, other: &PropositionSet) -> Self { Self(self.0.difference(&other.0).cloned().collect()) }

    /// Check if this [`PropositionSet`] contains the provided [`Proposition`]
    pub fn contains(&self, proposition: &Proposition) -> bool { self.0.contains(proposition) }
    /// Check if this [`PropositionSet`] contains every provided [`Proposition`]
    pub fn contains_all<'a>(&self, propositions: impl Iterator<Item=&'a Proposition>) -> bool {
        for proposition in propositions { if !self.contains(proposition) { return false; } }
        true
    }

    /// Check if this [`PropositionSet`] contains every [`Proposition`] in another provided [`PropositionSet`]
    pub fn subset_of(&self, other: &PropositionSet) -> bool { self.0.is_subset(&other.0) }

    /// Get the number of [`Proposition`] objects that this [`PropositionSet`] contains
    pub fn len(&self) -> usize { self.0.len() }
    /// Check if this [`PropositionSet`] contains no [`Proposition`] objects
    pub fn is_empty(&self) -> bool { self.0.is_empty() }
}

impl From<&Proposition> for PropositionSet {
    fn from(proposition: &Proposition) -> Self {
        Self([proposition.clone()].iter().cloned().collect())
    }
}
impl From<&Vec<Proposition>> for PropositionSet {
    fn from(starting_propositions: &Vec<Proposition>) -> Self {
        Self(starting_propositions.iter().cloned().collect())
    }
}
impl From<&HashSet<Proposition>> for PropositionSet {
    fn from(starting_propositions: &HashSet<Proposition>) -> Self {
        Self(starting_propositions.clone())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_merge() {
        
    }
}
