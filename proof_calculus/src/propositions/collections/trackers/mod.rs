use std::{collections::HashSet, hash::Hash};

use crate::propositions::{Proposition, bounds::{PropositionBoundsAssignedIdentity, PropositionBoundsAssignedInsertion, PropositionBoundsUnassignedSubsumesAssigned, unassigned::PropositionBoundsUnassignedSubsumedByUnassigned}, unassigned::UnassignedProposition};

// Feature: generation
pub mod unassigned;

pub trait BindGetPropositionsByBounds<P: Proposition, B> {
    type Value: Eq + Hash;

    fn get<'a>(&'a self, key: B) -> HashSet<&'a Self::Value>;
    fn get_all<'a>(&'a self) -> HashSet<&'a Self::Value>;
    fn get_intersection<'a, I: IntoIterator<Item=B>>(&'a self, bounds: I) -> HashSet<&'a Self::Value> {
        let mut iter = bounds.into_iter();
        if let Some(value) = iter.next() {
            let mut results = self.get(value);
            for bound in iter {
                if results.len() == 0 { break; }
                results = results.intersection(&self.get(bound)).map(|v| *v).collect();
            }
            results
        } else { self.get_all() }
    }
    
    fn get_identical<'a, 'b, PE: Proposition, I: PropositionBoundsAssignedIdentity<'a,PE,P,B>>(&'b self, bounds: I) -> Option<&'b Self::Value> where PE: 'a {
        let mut results = self.get_intersection(bounds).into_iter();
        let result = results.next();
        debug_assert!(results.next().is_some() ,"GetPropositionByBounds produced multiple outputs in PropositionTracker");
        result
    }
    fn get_subsumed_by<'a, 'b, PE: UnassignedProposition + 'a, I: PropositionBoundsUnassignedSubsumesAssigned<'a,PE,P,B>>(&'b self, bounds: I) -> HashSet<&'b Self::Value>
        { self.get_intersection(bounds) }
    
}

pub trait BindInsertPropositionByBounds<P: Proposition, B> {
    type Value: Eq + Hash;

    fn insert<'a, I: PropositionBoundsAssignedInsertion<'a,P,B>>(&'a mut self, bounds: I, value: Self::Value) where P: 'a;
}

pub trait BindInsertProposition<P: Proposition>: BindInsertPropositionByBounds<P,Self::DefaultInsertionBound> {
    type DefaultInsertionBound;
    type DefaultInsertionBounds<'a>: PropositionBoundsAssignedInsertion<'a,P,Self::DefaultInsertionBound> where P: 'a;
    
    fn insert_prop(&mut self, prop: &P, value: Self::Value)
        { self.insert(Self::DefaultInsertionBounds::from(prop), value) }
}

pub trait BindGetPropositionIdenticalToAssigned<PM: Proposition, PE: Proposition>: BindGetPropositionsByBounds<PM,Self::DefaultIdentityBound> {
    type DefaultIdentityBound;
    type DefaultIdentityBounds<'a>: PropositionBoundsAssignedIdentity<'a,PE,PM,Self::DefaultIdentityBound> where PE: 'a;
    
    fn get_identical_props(&self, prop: &PE) -> Option<&Self::Value>
        { self.get_identical(Self::DefaultIdentityBounds::from(prop)) }
}
pub trait BindGetPropositionsSubsumedByUnassigned<UP: UnassignedProposition>: BindGetPropositionsByBounds<UP::AssignedResult,Self::DefaultSubsumedByBound> {
    type DefaultSubsumedByBound;
    type DefaultSubsumedByBounds<'a>: PropositionBoundsUnassignedSubsumedByUnassigned<'a,UP,Self::DefaultSubsumedByBound>;
    
    fn get_subsumed_by<'a>(&'a self, unassigned_prop: &UP) -> HashSet<&'a Self::Value>
        { self.get_intersection(unassigned_prop.into()) }
}
