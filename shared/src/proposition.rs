use crate::entity::EntityId;

/// Error thrown by failing to properly construct a [Proposition]
pub enum PropositionConstructionError {
    EntityProvided, // Thrown when a Proposition is constructed with PropositionTerm::Entity instead of PropositionTerm::Tuple
    NotEnoughTermsProvided, // Thrown when a Proposition is constructed with a PropositionTerm::Tuple object which contains less than two terms
    ConstructedFromNonexistentTerm, // Thrown when proposition.proposition_from_term is called with an index that does not point to a tuple
}

/// A unit of truth under the system of Tuple-Based Logic
#[derive(Debug,Clone,PartialEq,Eq,Hash)]
pub struct Proposition(PropositionTerm);
impl Proposition {
    /// Create a new [Proposition] object, checking that it is a valid [Proposition]
    pub fn new(term: PropositionTerm) -> Result<Self,PropositionConstructionError> { 
        match &term {
            PropositionTerm::Entity(_) => Err(PropositionConstructionError::EntityProvided),
            PropositionTerm::Tuple(vec) => {
                if vec.len() < 2 { Err(PropositionConstructionError::NotEnoughTermsProvided) }
                else { Ok(Self(term)) }
            },
        }
    }

    /// Get the terms of this [Proposition], panicking if the [Proposition] is invalid 
    pub fn get_terms(&self) -> &Vec<PropositionTerm> { 
        match &self.0 {
            PropositionTerm::Entity(_) => panic!("Proposition contained an Entity, instead of a Tuple"),
            PropositionTerm::Tuple(proposition_terms) => proposition_terms,
        }
    }

    /// Get the term within this [Proposition] at the provided index
    pub fn get_term(&self, index: usize) -> Option<PropositionTerm> { self.get_terms().get(index).cloned() }
    /// Get the number of terms in this [Proposition]
    pub fn len(&self) -> usize { self.get_terms().len() }

    /// Create a new Proposition from the term at the provided index within this [Proposition]
    pub fn proposition_from_term(&self, index: usize) -> Result<Proposition,PropositionConstructionError> {
        if let Some(term) = self.get_term(index) {
            Proposition::new(term)
        } else { Err(PropositionConstructionError::ConstructedFromNonexistentTerm) }
        
    }
}

/// Components used in the construction of [Proposition] objects
#[derive(Debug,Clone,PartialEq,Eq,Hash)]
pub enum PropositionTerm {
    Entity(EntityId),
    Tuple(Vec<PropositionTerm>)
}
