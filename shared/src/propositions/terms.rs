use crate::atoms::{AtomId, BuiltInAtom};

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

    /// If this term is a Tuple, get its terms. Otherwise throw an error 
    pub fn as_slice(&self) -> Result<&[Term], ()> {
        match &self {
            Term::Atomic(_) => Err(()),
            Term::Tuple(proposition_terms) => Ok(proposition_terms.as_slice()),
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

#[cfg(test)]
mod tests {
    use super::*;
    use ids::Id16;

    #[test]
    fn test_as_atom_on_atom() {
        for i in 0..10 {
            let atomic_term = Term::from(AtomId(Id16(i)));
            assert_eq!(atomic_term.as_atom(), Ok(AtomId(Id16(i))));
        }
    }

    #[test]
    fn test_as_atom_on_tuple() {
        for i in 0..10 {
            let atomic_term = Term::from(vec![Term::from(AtomId(Id16(i)))]);
            assert_eq!(atomic_term.as_atom(), Err(()));
        }
    }

    #[test]
    fn test_as_tuple_on_atom() {
        for i in 0..10 {
            let atomic_term = Term::Atomic(AtomId(Id16(i)));
            assert_eq!(atomic_term.as_tuple(), Err(()));
        }
    }

    #[test]
    fn test_as_tuple_on_tuple() {
        for i in 0..10 {
            let atomic_term = Term::from(vec![Term::from(AtomId(Id16(i)))]);
            assert_eq!(atomic_term.as_tuple(), Ok(&vec![Term::from(AtomId(Id16(i)))]));
        }
    }

    #[test]
    fn test_as_slice_on_atom() {
        for i in 0..10 {
            let atomic_term = Term::Atomic(AtomId(Id16(i)));
            assert_eq!(atomic_term.as_slice(), Err(()));
        }
    }

    #[test]
    fn test_as_slice_on_tuple() {
        for i in 0..10 {
            let atomic_term = Term::from(vec![Term::from(AtomId(Id16(i)))]);
            assert_eq!(atomic_term.as_slice(), Ok(vec![Term::from(AtomId(Id16(i)))].as_slice()));
        }
    }

    #[test]
    fn test_get_subterm_on_atom() {
        for i in 0..10 {
            let atomic_term = Term::from(AtomId(Id16(i)));
            assert_eq!(atomic_term.get_subterm(0), Err(()));
        }
    }

    #[test]
    fn test_get_subterm_on_tuple() {
        for i in 0..10 {
            let atomic_term = Term::from(vec![Term::from(AtomId(Id16(i)))]);
            assert_eq!(atomic_term.get_subterm(0), Ok(&Term::from(AtomId(Id16(i)))));
        }
    }

    #[test]
    fn test_get_subterm_on_short_tuple() {
        for i in 0..10 {
            let atomic_term = Term::from(vec![Term::from(AtomId(Id16(i)))]);
            assert_eq!(atomic_term.get_subterm(1), Err(()));
        }
    }
}
