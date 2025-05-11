use crate::atom::{AtomId, BuiltInAtom};

/// Components used in the construction of [Proposition] objects
#[derive(Debug,Clone,PartialEq,Eq,Hash)]
pub enum Term {
    Atomic(AtomId),
    Tuple(Vec<Term>)
}

impl Term {
    // If this term is an Atom, get its id. Otherwise throw an error
    pub fn as_atom(&self) -> Result<AtomId,()> {
        match &self {
            Term::Atomic(entity_id) => Ok(*entity_id),
            Term::Tuple(_) => Err(()),
        }
    }

    /// If this term is a Tuple, get its terms. Otherwise throw an error 
    pub fn as_tuple(&self) -> Result<&Vec<Term>,()> { 
        match &self {
            Term::Atomic(_) => Err(()),
            Term::Tuple(proposition_terms) => Ok(proposition_terms),
        }
    }

    /// Get the term within this term at the provided index if it exists, otherwise throw an error.
    pub fn get_subterm(&self, index: usize) -> Result<&Term,()> {
        let terms= self.as_tuple()?;
        match terms.get(index) {
            Some(term) => Ok(term),
            None => Err(()),
        }
    }
}

impl From<AtomId> for Term {
    fn from(id: AtomId) -> Self { Self::Atomic(id) }
}

impl From<Vec<Term>> for Term {
    fn from(terms: Vec<Term>) -> Self { Self::Tuple(terms) }
}
impl From<BuiltInAtom> for Term {
    fn from(atom: BuiltInAtom) -> Self { Self::from(AtomId::from(atom.into())) }
}