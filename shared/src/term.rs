use crate::atom::AtomId;

/// Components used in the construction of [Proposition] objects
#[derive(Debug,Clone,PartialEq,Eq,Hash)]
pub enum Term {
    Atomic(AtomId),
    Tuple(Vec<Term>)
}

impl Term {
    // If this term is an Entity, get its id. Otherwise throw an error
    pub fn as_entity(&self) -> Result<AtomId,()> {
        match &self {
            Term::Atomic(entity_id) => Ok(*entity_id),
            Term::Tuple(_) => Err(()),
        }
    }

    /// If this term is a Tuple, get its terms. Otherwise throw an error 
    pub fn as_terms(&self) -> Result<&Vec<Term>,()> { 
        match &self {
            Term::Atomic(_) => Err(()),
            Term::Tuple(proposition_terms) => Ok(proposition_terms),
        }
    }

    /// Get the term within this term at the provided index if it exists, otherwise throw an error.
    pub fn get_term(&self, index: usize) -> Result<&Term,()> {
        let terms= self.as_terms()?;
        match terms.get(index) {
            Some(term) => Ok(term),
            None => Err(()),
        }
    }
}