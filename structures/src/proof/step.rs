use path_lib::{HasChildren, obj_at_path::{ObjAtPath, OwnedObjAtPath}};

use crate::{expressions::Proposition, proof::{AtomicProofInProofPath, Proof, inference::{InferenceRule, PropositionInInference, PropositionInInferencePath}}};

pub trait ProofStep<Rule:InferenceRule> : HasChildren<PropositionInInferencePath,Proposition> + HasChildren<AtomicProofInProofPath,Proof<Rule>> {
    // To be implemented by implemntors of this trait
    /// Get the [ProofPropositionPaths](ProofPropositionPath) of all assumptions within this [ProofStep]
    fn assumption_paths(&self) -> impl IntoIterator<Item = PropositionInInferencePath>;
    /// Get the [ProofPropositionPaths](ProofPropositionPath) of all explicit conclusions within this [ProofStep]
    fn explicit_conclusion_paths(&self) -> impl IntoIterator<Item = PropositionInInferencePath>;
    
    /// Get references to all assumptions within this [ProofStep]
    fn get_assumptions(&self) -> impl IntoIterator<Item = &Proposition>
        { self.assumption_paths().into_iter().map(|p| self.get_child(&p).unwrap()) }
    /// Get all assumptions within this [ProofStep]
    fn get_assumptions_owned(&self) -> impl IntoIterator<Item = Proposition>
        { self.assumption_paths().into_iter().map(|p| self.get_child(&p).unwrap().to_owned()) }
    /// Get references to all assumptions within this [ProofStep], located by their [ProofPropositionPath]
    fn get_located_assumptions<'a>(&'a self) -> impl IntoIterator<Item = ObjAtPath<'a,Proposition,PropositionInInferencePath>>
        { self.assumption_paths().into_iter().map(|p| self.get_located_child(p).unwrap()) }
    /// Get all assumptions within this [ProofStep], located by their [ProofPropositionPath]
    fn get_located_assumptions_owned(&self) -> impl IntoIterator<Item = OwnedObjAtPath<Proposition,PropositionInInferencePath>>
        { self.assumption_paths().into_iter().map(|p| self.get_located_child_owned(p).unwrap()) }
    
    /// Get all explicit conclusions within this [ProofStep]
    fn get_explicit_conclusions(&self) -> impl IntoIterator<Item = &Proposition>
        { self.explicit_conclusion_paths().into_iter().map(|p| self.get_child(&p).unwrap()) }
    /// Get owned versions of all explicit conclusions within this [ProofStep]
    fn get_explicit_conclusions_owned(&self) -> impl IntoIterator<Item = Proposition>
        { self.explicit_conclusion_paths().into_iter().map(|p| self.get_child(&p).unwrap().to_owned()) }
    /// Get all explicit conclusions within this [ProofStep], located by their [ProofPropositionPath]
    fn get_located_explicit_conclusions<'a>(&'a self) -> impl IntoIterator<Item = PropositionInInference<'a>>
        { self.explicit_conclusion_paths().into_iter().map(|p| self.get_located_child(p).unwrap().into()) }
    /// Get owned versions of all explicit conclusions within this [ProofStep], located by their [ProofPropositionPath]
    fn get_located_explicit_conclusions_owned(&self) -> impl IntoIterator<Item = OwnedObjAtPath<Proposition,PropositionInInferencePath>>
        { self.explicit_conclusion_paths().into_iter().map(|p| self.get_located_child_owned(p).unwrap()) }
}

pub (crate) fn valid_primitive_paths_inner<Rule: InferenceRule, Step: ProofStep<Rule>>(step: &Step, num_conclusions: usize) -> Vec<PropositionInInferencePath> {
    let assumptions = (0..step.assumption_paths().into_iter().count()).map(|ix| PropositionInInferencePath::assumption(ix));
    let conclusions = (0..num_conclusions).map(|ix| PropositionInInferencePath::conclusion(ix));
    assumptions.chain(conclusions).collect()
}
pub (crate) fn get_child_inner<'a,Rule: InferenceRule, Step: ProofStep<Rule>>(step: &'a Step, path: &PropositionInInferencePath) -> Result<&'a Proposition,()> {
    let n = path.proposition_index;
    if path.is_conclusion { step.get_assumptions().into_iter().nth(n) } 
    else { step.get_explicit_conclusions().into_iter().nth(n) }
        .ok_or(())
}
