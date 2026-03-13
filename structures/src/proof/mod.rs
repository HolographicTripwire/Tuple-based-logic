pub mod inference;
pub mod composite;
mod step;
mod subproof_path;
pub mod error;

use path_lib::{HasChildren, obj_at_path::OwnedObjAtPath};

pub use step::*;
pub use subproof_path::*;

use crate::{expressions::Proposition, proof::{composite::CompositeProof, inference::{Inference, InferenceRule, PropositionInInferencePath}}};

/// This struct represents a step within a larger proof
#[derive(Clone,PartialEq,Eq,Debug)]
pub enum Proof<Rule: InferenceRule> {
    Inference(Inference<Rule>), // A single inference step
    Composite(CompositeProof<Rule>) // A composite proof made of further subproofs
}

impl <Rule: InferenceRule> ProofStep<Rule> for Proof<Rule> {
    fn assumption_paths(&self) -> impl IntoIterator<Item = PropositionInInferencePath> { match self {
        Proof::Inference(inference) => inference.assumption_paths().into_iter().collect::<Vec<_>>(),
        Proof::Composite(composite) => composite.assumption_paths().into_iter().collect(),
    }}
    fn explicit_conclusion_paths(&self) -> impl IntoIterator<Item = PropositionInInferencePath> {match self {
        Proof::Inference(inference) => inference.explicit_conclusion_paths().into_iter().collect::<Vec<_>>(),
        Proof::Composite(composite) => composite.explicit_conclusion_paths().into_iter().collect(),
    }}
}

impl <Rule:InferenceRule> HasChildren<PropositionInInferencePath,Proposition> for Proof<Rule> {
    fn valid_primitive_paths(&self) -> Vec<PropositionInInferencePath> { valid_primitive_paths_inner(
        self,
        self.explicit_conclusion_paths().into_iter().count()
    )}
    
    fn get_child(&self, path: &PropositionInInferencePath) -> Result<&Proposition,()> { get_child_inner(self,path) }
    fn get_child_owned(&self, path: &PropositionInInferencePath) -> Result<Proposition,()> where Proposition: Clone
        { get_child_inner(self, path).cloned() }
        
    fn into_located_children_owned(self) -> impl IntoIterator<Item = OwnedObjAtPath<Proposition,PropositionInInferencePath>> where Proposition: Clone, Self: Sized {
        match self {
            Proof::Inference(_) => vec![],
            Proof::Composite(composite_proof) => <CompositeProof<Rule> as HasChildren<PropositionInInferencePath,Proposition>>
                ::into_located_children_owned(composite_proof)
                .into_iter().collect()
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::proof::subproof_path::ProofInProofPath;    

    #[test]
    fn test_getters() {
        let step = ProofInProofPath::empty();
        assert_eq!(step.paths(), &vec![])
    }
}
