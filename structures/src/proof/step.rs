use std::collections::HashSet;

use path_lib::{HasChildren, obj_at_path::{ObjAtPath, OwnedObjAtPath}};

use crate::{expressions::{Proposition, PropositionSet}, proof::{AtomicProofInProofPath, ImmediateProofInProof, OwnedImmediateProofInProof, Proof, inference::{InferenceRule, PropositionInInference, PropositionInInferencePath}}};

pub trait ProofStep<Rule:InferenceRule> : HasChildren<PropositionInInferencePath,Proposition> + HasChildren<AtomicProofInProofPath,Proof<Rule>> {
    // To be implemented by implemntors of this trait
    /// Get the [ProofPropositionPaths](ProofPropositionPath) of all assumptions within this [ProofStep]
    fn assumption_paths(&self) -> impl IntoIterator<Item = PropositionInInferencePath>;
    /// Get the [ProofPropositionPaths](ProofPropositionPath) of all explicit conclusions within this [ProofStep]
    fn explicit_conclusion_paths(&self) -> impl IntoIterator<Item = PropositionInInferencePath>;
    /// Get the [AtomicSubproofPaths](AtomicSubproofPath) of all immediate subproofs within this [ProofStep]
    fn immediate_subproof_paths<'a>(&'a self) -> impl IntoIterator<Item=AtomicProofInProofPath> { <Self as HasChildren<AtomicProofInProofPath,Proof<Rule>>>::valid_primitive_paths(self) }
    
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
    
    /// Get all immediate subproofs within this [ProofStep]
    fn get_immediate_subproofs(&self) -> impl IntoIterator<Item = &Proof<Rule>>
        { self.immediate_subproof_paths().into_iter().map(|p| self.get_child(&p).unwrap()) }
    /// Get owned versions of all immediate subproofs within this [ProofStep]
    fn get_immediate_subproofs_owned(&self) -> impl IntoIterator<Item = Proof<Rule>>
        { self.immediate_subproof_paths().into_iter().map(|p| self.get_child(&p).unwrap().to_owned()) }
    /// Get all immediate subproofs within this [ProofStep], located by their [AtomicSubproofPath]
    fn get_located_immediate_subproofs<'a>(&'a self) -> impl IntoIterator<Item = ImmediateProofInProof<'a,Rule>>
        { self.immediate_subproof_paths().into_iter().map(|p| self.get_located_child(p).unwrap().into()) }
    /// Get owned versions of all immediate subproofs within this [ProofStep], located by their [AtomicSubproofPath]
    fn get_located_immediate_subproofs_owned(&self) -> impl IntoIterator<Item = OwnedImmediateProofInProof<Rule>>
        { self.immediate_subproof_paths().into_iter().map(|p| self.get_located_child_owned(p).unwrap().into()) }

    /// Get all conclusions of this [ProofStep]
    fn get_conclusions(&self) -> HashSet<&Proposition> {
        self.get_immediate_subproofs().into_iter().fold(
            // Combine
            HashSet::from_iter(self.get_explicit_conclusions()), // The explcit conclusions of this ProofStep
            |mut acc,next| { acc.extend(next.get_conclusions().iter()); acc } // And the conclusions of this expression's children Subproofs
        )
    }
    /// Get owned versions of all conclusions of this [ProofStep]
    fn get_conclusions_owned(&self) -> PropositionSet {
        self.get_immediate_subproofs().into_iter().fold(
            // Combine
            PropositionSet::from_iter(self.get_explicit_conclusions_owned()), // The explcit conclusions of this ProofStep
            |mut acc,next| { acc.extend(next.get_conclusions_owned().into_iter()); acc } // And the conclusions of this expression's children Subproofs
        )
    }
    
    /// Get all implicit conclusions of this [ProofStep]
    fn get_implicit_conclusions(&self) -> HashSet<&Proposition> {
        let explicits = HashSet::from_iter(self.get_explicit_conclusions());
        self.get_conclusions().difference(&explicits).cloned().collect()
    }
    /// Get owned versions of all implicit conclusions of this [ProofStep]
    fn get_implicit_conclusions_owned(&self) -> PropositionSet {
        let explicits = PropositionSet::from_iter(self.get_explicit_conclusions_owned());
        self.get_conclusions_owned().difference(&explicits).cloned().collect()
    }
}
pub (crate) fn valid_primitive_paths_inner<Rule: InferenceRule, Step: ProofStep<Rule>>(step: &Step) -> Vec<PropositionInInferencePath> {
    let assumptions = (0..step.assumption_paths().into_iter().count()).map(|ix| PropositionInInferencePath::assumption(ix));
    let conclusions = (0..step.get_conclusions().len()).map(|ix| PropositionInInferencePath::conclusion(ix));
    assumptions.chain(conclusions).collect()
}
pub (crate) fn get_child_inner<'a,Rule: InferenceRule, Step: ProofStep<Rule>>(step: &'a Step, path: &PropositionInInferencePath) -> Result<&'a Proposition,()> {
    let n = path.proposition_index;
    if path.is_conclusion { step.get_assumptions().into_iter().nth(n) } 
    else { step.get_explicit_conclusions().into_iter().nth(n) }
    .ok_or(())
}
