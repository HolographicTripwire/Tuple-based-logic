mod signatures;
mod sources;

use std::collections::HashMap;

use tbl_structures::{atoms::AtomId, expressions::Proposition};
use tbl_generation::PropositionQuery;

/// A [CorePropositionQuery] is a [PropositionQuery] that is used in projects that build from the tbl_core crate
/// 
/// Every [CorePropositionQuery] is a [Proposition], where certain components may have been replaced with unbound variables.
/// 
/// Example (variables prefixed with '?'): (=, ?0, ?1) represents a query for any Proposition that asserts the identity of two entities (e.g. (=, Batman, BruceWayne))
/// 
/// A [CorePropositionQuery] is guaranteed to be normalised, meaning that its variables will be numbered in the order that they appear. For a non-normalised form, see [CorePropositionQueryInner]
struct CorePropositionQuery {
    inner: CorePropositionQueryInner
}

impl CorePropositionQuery {
    fn new(inner: CorePropositionQueryInner) -> Self
        { Self { inner: inner.normalise() } }
}

impl From<Proposition> for CorePropositionQuery {
    fn from(p: Proposition) -> Self
        { Self::new(p.into()).unwrap("Proposition could not be converted into a PropositionQuery") }
}

/// A [CorePropositionQuery] which may or may not be normalised
pub enum CorePropositionQueryInner {
    AtomicProposition(AtomId),
    Variable(usize),
    Tuple(Vec<CorePropositionQueryInner>)
}

impl CorePropositionQueryInner {
    /// Create a new [CorePropositionQueryInner] which is guaranteed to be normalised
    pub fn normalise(mut self) { self.normalise_inner(0, HashMap::new()) }
    
    /// A helper method for [CorePropositionQueryInner::normalise]
    fn normalise_inner(mut self, mut next_index: usize, mut var_mappings: HashMap<usize,usize>) {
        match self {
            // We don't need to change atomic propositions
            CorePropositionQueryInner::AtomicProposition(atom_id) => {},
            // Map variables to their normalised index
            CorePropositionQueryInner::Variable(mut i) => {
                // If this variable already has a mapping, change it to the mapped value
                if let Some(k) = var_mappings.get(i) { i = k }
                // Otherwise assign it the next available index
                else {
                    var_mappings.insert(i, next_index);
                    i = next_index;
                    next_index += 1;
                }},
            // Perform normalisation on each component of any tuple
            CorePropositionQueryInner::Tuple(items) => 
                items
                    .into_iter()
                    .for_each(|q| q.normalise_inner(next_index, var_mappings)),
        }
    }
}

impl From<Proposition> for CorePropositionQueryInner {
    fn from(p: Proposition) -> Self {
        match p {
            Proposition::Atomic(atom_id) => Self::AtomicProposition(atom_id),
            Proposition::Tuple(expressions) => Self::Tuple(expressions.iter().map(|e| e.into()).collect()),
        }
    }
}
