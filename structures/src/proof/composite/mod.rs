use path_lib::obj_at_path::{ObjAtPath, OwnedObjAtPath};

use crate::{expressions::Proposition, proof::{AssumptionInProofStepPath, ExplicitConclusionInProofStepPath, ParentOfAssumptions, ParentOfExplicitConclusions, ParentOfSubproofs, Proof, ProofInProofPath, at_path_enum::ProofAtPathEnum, immediate::{ImmediateProofInProofPath, ParentOfImmediateSubproof}, inference::{Inference, InferenceRule}}};

#[derive(Clone,PartialEq,Eq,Debug)]
pub struct CompositeProof<Rule: InferenceRule> {
    pub assumptions: Vec<Proposition>,
    pub subproofs: Vec<Proof<Rule>>,
    pub explicit_conclusions: Vec<Proposition>,
}
impl <Rule: InferenceRule> CompositeProof<Rule> {
    pub fn new(assumptions: Vec<Proposition>, subproofs: Vec<Proof<Rule>>, explicit_conclusions: Vec<Proposition>) -> Self
        { Self { assumptions, subproofs, explicit_conclusions } }
    // /// Get the [AtomicSubproofPaths](AtomicSubproofPath) of all immediate subproofs within this [ProofStep]
    // pub fn immediate_subproof_paths<'a>(&'a self) -> impl IntoIterator<Item=AtomicProofInProofPath> { <Self as HasChildren<AtomicProofInProofPath,Proof<Rule>>>::valid_primitive_paths(self) }
    
    // /// Get all immediate subproofs within this [ProofStep]
    // pub fn get_immediate_subproof(&self, step: AtomicProofInProofPath) -> Result<&Proof<Rule>,()> { self.get_child(&step) }
    // pub fn get_immediate_subproofs(&self) -> impl IntoIterator<Item = &Proof<Rule>>
    //     { self.immediate_subproof_paths().into_iter().map(|p| self.get_immediate_subproof(p).unwrap()) }
    // /// Get owned versions of all immediate subproofs within this [ProofStep]
    // pub fn get_immediate_subproof_owned(&self, step: AtomicProofInProofPath) -> Result<Proof<Rule>,()> { self.get_child(&step).cloned() }
    // pub fn get_immediate_subproofs_owned(&self) -> impl IntoIterator<Item = Proof<Rule>>
    //     { self.immediate_subproof_paths().into_iter().map(|p| self.get_immediate_subproof_owned(p).unwrap()) }
    // /// Get all immediate subproofs within this [ProofStep], located by their [AtomicSubproofPath]
    // pub fn get_located_immediate_subproof<'a>(&'a self, step: AtomicProofInProofPath) -> Result<ImmediateProofInProof<'a,Rule>,()>
    //     { self.get_located_child(step).map(|x| x.into()) }
    // pub fn get_located_immediate_subproofs<'a>(&'a self) -> impl IntoIterator<Item = ImmediateProofInProof<'a,Rule>>
    //     { self.immediate_subproof_paths().into_iter().map(|p| self.get_located_immediate_subproof(p).unwrap()) }
    // /// Get owned versions of all immediate subproofs within this [ProofStep], located by their [AtomicSubproofPath]
    // pub fn get_located_immediate_subproof_owned(&self, step: AtomicProofInProofPath) -> Result<OwnedImmediateProofInProof<Rule>,()>
    //     { self.get_located_child_owned(step).map(|x| x.into())}
    // pub fn get_located_immediate_subproofs_owned(&self) -> impl IntoIterator<Item = OwnedImmediateProofInProof<Rule>>
    //     { self.immediate_subproof_paths().into_iter().map(|p| self.get_located_immediate_subproof_owned(p).unwrap()) }

    // /// Get all conclusions of this [ProofStep]
    // pub fn get_conclusions(&self) -> HashSet<&Proposition> {
    //     self.get_immediate_subproofs().into_iter().fold(
    //         // Combine
    //         HashSet::from_iter(self.get_explicit_conclusions()), // The explcit conclusions of this ProofStep
    //         |mut acc,next| {
    //             match next {
    //                 Proof::Inference(inference) => acc.extend(inference.get_explicit_conclusions()),
    //                 Proof::Composite(composite) => acc.extend(composite.get_conclusions().into_iter()),
    //             } acc
    //         } // And the conclusions of this expression's children Subproofs
    //     )
    // }
    // /// Get owned versions of all conclusions of this [ProofStep]
    // pub fn get_conclusions_owned(&self) -> PropositionSet {
    //     self.get_immediate_subproofs().into_iter().fold(
    //         // Combine
    //         PropositionSet::from_iter(self.get_explicit_conclusions_owned()), // The explcit conclusions of this ProofStep
    //         |mut acc,next| {
    //             match next {
    //                 Proof::Inference(inference) => acc.extend(inference.get_explicit_conclusions_owned()),
    //                 Proof::Composite(composite) => acc.extend(composite.get_conclusions_owned().into_iter()),
    //             }; acc
    //         } // And the conclusions of this expression's children Subproofs
    //     )
    // }
    
    // /// Get all implicit conclusions of this [ProofStep]
    // pub fn get_implicit_conclusions(&self) -> HashSet<&Proposition> {
    //     let explicits = HashSet::from_iter(self.get_explicit_conclusions());
    //     self.get_conclusions().difference(&explicits).cloned().collect()
    // }
    // /// Get owned versions of all implicit conclusions of this [ProofStep]
    // pub fn get_implicit_conclusions_owned(&self) -> PropositionSet {
    //     let explicits = PropositionSet::from_iter(self.get_explicit_conclusions_owned());
    //     self.get_conclusions_owned().difference(&explicits).cloned().collect()
    // }
}
impl <Rule: InferenceRule> ParentOfAssumptions for CompositeProof<Rule> {
    fn get_assumption_paths(&self) -> impl IntoIterator<Item = AssumptionInProofStepPath>
        { (0..self.assumptions.len()).map(|n| AssumptionInProofStepPath(n)) }

    fn get_assumption(&self,path: &AssumptionInProofStepPath) -> Result< &Proposition,()> {
        self.assumptions.get(path.0).ok_or(())
    }
    
    fn get_assumptions(&self) -> impl IntoIterator<Item = &Proposition> { &self.assumptions }
}
impl <Rule: InferenceRule> ParentOfExplicitConclusions for CompositeProof<Rule> {
    fn get_explicit_conclusion_paths(&self) -> impl IntoIterator<Item = ExplicitConclusionInProofStepPath> 
        { (0..self.explicit_conclusions.len()).map(|n| ExplicitConclusionInProofStepPath(n)) }

    fn get_explicit_conclusion(&self,path: &ExplicitConclusionInProofStepPath) -> Result< &Proposition,()>  {
        self.explicit_conclusions.get(path.0).ok_or(())
    }

    fn get_explicit_conclusions(&self) -> impl IntoIterator<Item = &Proposition> { &self.explicit_conclusions }
}

impl <Rule:InferenceRule> ParentOfImmediateSubproof<Rule> for CompositeProof<Rule> {
    fn get_immediate_subproof_paths(&self) -> impl IntoIterator<Item = ImmediateProofInProofPath>
        { (0..self.subproofs.len()).map(|ix| ix.into()) }
    fn get_immediate_subproof(&self,path: &ImmediateProofInProofPath) -> Result< &Proof<Rule> ,()> 
        { self.subproofs.get(path.0).ok_or(()) }
    fn into_located_immediate_subproofs_owned(self) -> impl IntoIterator<Item = OwnedObjAtPath<Proof<Rule>,ImmediateProofInProofPath>> where Proof<Rule>: Clone, Self: Sized {
        self.subproofs.into_iter()
            .enumerate()
            .map(|(id,proof)| OwnedObjAtPath{obj: proof, path: id.into()})
    }
}

impl <Rule: InferenceRule> CompositeProof<Rule> {
    fn get_subproofs_helper(&self,path: &ProofInProofPath, index: usize) -> Result<&Proof<Rule>,()> {
        let immediate_path = path.0.get(index).ok_or(())?;
        let inner = self.get_immediate_subproof(immediate_path)?;
        if index == path.0.len() { Ok(inner) }
        else { match inner {
            Proof::Inference(_) => Err(()),
            Proof::Composite(composite) => composite.get_subproofs_helper(path, index+1),
        }}
    }
}
impl <Rule: InferenceRule> ParentOfSubproofs<Rule> for CompositeProof<Rule> {
    fn get_subproof_paths(&self) -> impl IntoIterator<Item = ProofInProofPath>  {
        let immediate = self.get_immediate_subproof_paths()
            .into_iter()
            .map(|x| x.into());
        let deferred = self.get_located_immediate_subproofs()
            .into_iter()
            .map(|x| match x.into() {
                ProofAtPathEnum::Inference(_) => vec![],
                ProofAtPathEnum::Composite(composite) => composite.obj
                    .get_subproof_paths()
                    .into_iter()
                    .map(|p| (composite.path,p).into())
                    .collect()
                }
            ).flatten();
        immediate.chain(deferred)
    }

    fn get_subproof(&self,path: &ProofInProofPath) -> Result<&Proof<Rule>,()>
        { self.get_subproofs_helper(path, 0) }
}

pub type InferenceInProof<'a, Rule> = ObjAtPath<'a,Inference<Rule>,ProofInProofPath>;
pub type OwnedInferenceInProof<Rule> = OwnedObjAtPath<Inference<Rule>,ProofInProofPath>;
