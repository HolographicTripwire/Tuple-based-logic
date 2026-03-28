use std::hash::Hash;

pub mod propositions;
pub mod inferences;
pub mod abstract_proofs;
pub mod sequential_proofs;

pub trait Proposition: Clone + PartialEq + Eq + Hash {}
