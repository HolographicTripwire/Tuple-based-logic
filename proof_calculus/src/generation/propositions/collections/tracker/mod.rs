use std::collections::HashSet;

use crate::{generation::propositions::{UnassignedProposition, bounds::{PropositionBoundsUnassignedIdentity, PropositionBoundsUnassignedSubsumedByUnassigned, PropositionBoundsUnassignedSubsumesUnassigned, UnassignedPropositionBound, UnassignedPropositionIdentityBounds, UnassignedPropositionSubsumedByBounds}}, structures::propositions::{Proposition, bounds::PropositionBoundsAssignedIdentity}};

// pub trait UnassignedPropositionTracker<P: UnassignedProposition> {
//     type Value;

//     fn insert(&mut self, key: P, value: Self::Value);
// }

// pub trait UnassignedPropositionIdenticalTracker<P: UnassignedProposition, B: UnassignedPropositionBound<P>>: UnassignedPropositionTracker<P> {
//     type DefaultIdentityBounds: UnassignedPropositionIdentityBounds<P,B>;
    
//     fn get_identical(&self, prop: &P) -> Option<&Self::Value> { self.get_identical2(Self::DefaultIdentityBounds::new(prop)) }
//     fn get_identical2<I: UnassignedPropositionIdentityBounds<P,B>>(&self, i: I) -> Option<&Self::Value>;
// }
// pub trait UnassignedPropositionSubsumedByTracker<UP: UnassignedProposition, B: UnassignedPropositionBound<UP>>: UnassignedPropositionTracker<UP> {
//     fn get_subsumed_by<'a, I: UnassignedPropositionSubsumedByBounds<UP,B>>(&'a self, i: I) -> impl IntoIterator<Item=&'a Self::Value> where Self::Value: 'a;
// }

// pub trait UnassignedPropositionTracker<P: UnassignedProposition, B> {
//     type Value;

//     fn get<'a>(&'a self, key: B) -> HashSet<&'a Self::Value>;
//     fn get_all<'a>(&'a self) -> HashSet<&'a Self::Value>;
//     fn get_intersection<'a, I: IntoIterator<Item=B>>(&'a self, bounds: I) -> HashSet<&'a Self::Value> {
//         let mut iter = bounds.into_iter();
//         if let Some(value) = iter.next() {
//             let mut results = self.get(value);
//             for bound in iter { results = results.intersection(self.get(bound)); }
//             results
//         } else { self.get_all() }
//     }
    
//     fn get_identical<'a, I: PropositionBoundsUnassignedIdentity<'a,P,B>>(&'a self, bounds: I) -> Option<&'a Self::Value> {
//         let mut results = self.get_identical(bounds).into_iter();
//         let result = results.next();
//         debug_assert!(results.next().is_some() ,"PropositionBoundsAssignedIdentity produced multiple outputs in PropositionTracker");
//         result
//     }
//     fn get_subsumed_by<'a, I: PropositionBoundsUnassignedSubsumedByUnassigned<'a,P,B>>(&'a self, bounds: I) -> HashSet<&'a Self::Value>
//         { self.get_intersection(bounds) }
//     fn get_subsumes<'a, I: PropositionBoundsUnassignedSubsumesUnassigned<'a,P,B>>(&'a self, bounds: I) -> HashSet<&'a Self::Value>
//         { self.get_intersection(bounds) }
// }

// pub trait PropositionTrackerAssignedIdentity<P: Proposition>: UnassignedPropositionTracker<P,Self::DefaultIdentityBound> {
//     type DefaultIdentityBound;
//     type DefaultIdentityBounds<'a>: PropositionBoundsAssignedIdentity<'a,P,Self::DefaultIdentityBound>;
    
//     fn get_identical_prop(&self, prop: &P) -> Option<&Self::Value>
//         { self.get_identical(prop.into()) }
// }
// pub trait PropositionTrackerAssignedSubsumedByUnassigned<UP: UnassignedProposition>: UnassignedPropositionTracker<UP::AssignedResult,Self::DefaultSubsumedByBound> {
//     type DefaultSubsumedByBound;
//     type DefaultSubsumedByBounds<'a>: PropositionBoundsUnassignedSubsumedByUnassigned<'a,UP,Self::DefaultSubsumedByBound>;
    
//     fn get_subsumed_by_unassigned_prop<'a>(&'a self, unassigned_prop: &UP) -> HashSet<&'a Self::Value>
//         { self.get_intersection(unassigned_prop.into()) }
// }
