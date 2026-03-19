use path_lib::obj_at_path::{ObjAtPath, OwnedObjAtPath};

use crate::{expressions::Proposition, proof::{AssumptionInProofStepPath, AtomicProofInProofPath, ExplicitConclusionInProofStepPath, ParentOfAssumptions, ParentOfExplicitConclusions, Proof, ProofInProofPath, immediate::ParentOfImmediateSubproof, inference::{Inference, InferenceRule}}};

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
    fn get_immediate_subproof_paths(&self) -> impl IntoIterator<Item = AtomicProofInProofPath>
        { (0..self.subproofs.len()).map(|ix| ix.into()) }
    fn get_immediate_subproof(&self,path: &AtomicProofInProofPath) -> Result< &Proof<Rule> ,()> 
        { self.subproofs.get(path.0).ok_or(()) }
    fn into_located_immediate_subproofs_owned(self) -> impl IntoIterator<Item = OwnedObjAtPath<Proof<Rule>,AtomicProofInProofPath>> where Proof<Rule>: Clone, Self: Sized {
        self.subproofs.into_iter()
            .enumerate()
            .map(|(id,proof)| OwnedObjAtPath{obj: proof, path: id.into()})
    }
}

// impl <Rule:InferenceRule> HasChildren<PropositionInProofStepPath,Proposition> for CompositeProof<Rule> {
//     fn valid_primitive_paths(&self) -> Vec<PropositionInProofStepPath> { valid_primitive_paths_inner(self, self.explicit_conclusions.len()) }
    
//     fn get_child(&self, path: &PropositionInProofStepPath) -> Result<&Proposition,()> { get_child_inner(self,path) }
//     fn get_child_owned(&self, path: &PropositionInProofStepPath) -> Result<Proposition,()> where Proposition: Clone 
//         { get_child_inner(self,path).cloned() }
        
//     fn into_located_children_owned(self) -> impl IntoIterator<Item = OwnedObjAtPath<Proposition,PropositionInProofStepPath>> where Proposition: Clone, Self: Sized {
//         let assumptions = self.assumptions
//             .into_iter()
//             .enumerate()
//             .map(|(id,prop)| OwnedObjAtPath::from_inner(prop,PropositionInProofStepPath::assumption(id)));
//         let explicit_conclusions = self.explicit_conclusions.into_iter()
//             .enumerate()
//             .map(|(id, prop)| OwnedObjAtPath::from_inner(prop,PropositionInProofStepPath::explicit_conclusion(id)));
//         return assumptions.chain(explicit_conclusions)
//     }
// }
// impl <Rule:InferenceRule> HasChildren<AtomicProofInProofPath,Proof<Rule>> for CompositeProof<Rule> {
//     fn valid_primitive_paths(&self) -> Vec<AtomicProofInProofPath> 
//         { (0..self.subproofs.len()).map(|ix| ix.into()).collect() }
//     fn get_child(&self, path: &AtomicProofInProofPath) -> Result<&Proof<Rule>,()>
//         { self.subproofs.get(path.0).ok_or(()) }
//     fn get_child_owned(&self, path: &AtomicProofInProofPath) -> Result<Proof<Rule>,()> where Proof<Rule>: Clone
//         { self.subproofs.get(path.0).ok_or(()).cloned() }
// }

pub type InferenceInProof<'a, Rule> = ObjAtPath<'a,Inference<Rule>,ProofInProofPath>;
pub type OwnedInferenceInProof<Rule> = OwnedObjAtPath<Inference<Rule>,ProofInProofPath>;

// impl <'a,Rule: InferenceRule> CompositeProofInProof<'a,Rule> {
//     fn replace_immediate_path(&self, p: ImmediateProofInProof<'a,Rule>) -> ProofInProof<'a,Rule> {
//         p.replace_path(|p| {
//             let mut p1 = self.path().clone();
//             p1.append(p);
//             p1
//         }).into()
//     }
//     fn replace_immediate_owned_path(&self, p: OwnedImmediateProofInProof<Rule>) -> OwnedProofInProof<Rule> {
//         p.replace_path(|p| {
//             let mut p1 = self.path().clone();
//             p1.append(p);
//             p1
//         }).into()
//     }
    
//     pub fn get_located_immediate_subproof(&'a self, step: AtomicProofInProofPath) -> Result<ProofInProof<'a,Rule>,()>
//         { self.obj().get_located_immediate_subproof(step).map(|p| self.replace_immediate_path(p)) }
//     pub fn get_located_immediate_subproofs(&'a self) -> impl IntoIterator<Item = ProofInProof<'a, Rule>>
//         { self.obj().get_located_immediate_subproofs().into_iter().map(|p| {self.replace_immediate_path(p)}) }
//     pub fn get_located_immediate_subproof_owned(&self, step: AtomicProofInProofPath) -> Result<OwnedProofInProof<Rule>,()>
//         { self.obj().get_located_immediate_subproof_owned(step).map(|p| self.replace_immediate_owned_path(p)) }
//     pub fn get_located_immediate_subproofs_owned(&self) -> impl IntoIterator<Item = OwnedProofInProof<Rule>>
//         { self.obj().get_located_immediate_subproofs_owned().into_iter().map(|p| self.replace_immediate_owned_path(p)) }
// }
// impl <'a,Rule: InferenceRule> CompositeProofInProof<'a,Rule> {
    
// }
// impl <Rule: InferenceRule> OwnedCompositeProofInProof<Rule> {
//     pub fn get_located_immediate_subproofs<'a>(&'a self) -> impl IntoIterator<Item = ProofInProof<'a, Rule>> {
//         self.obj().get_located_immediate_subproofs().into_iter().map(|p| {
//             p.replace_path(|p| {
//                 let mut p1 = self.path().clone();
//                 p1.append(p);
//                 p1
//             }).into()
//         })
//     }
//     pub fn get_located_immediate_subproofs_owned(&self) -> impl IntoIterator<Item = OwnedProofInProof<Rule>> {
//         self.obj().get_located_immediate_subproofs_owned().into_iter().map(|p| {
//             p.replace_path(|p| {
//                 let mut p1 = self.path().clone();
//                 p1.append(p);
//                 p1
//             }).into()
//         })
//     }
// }