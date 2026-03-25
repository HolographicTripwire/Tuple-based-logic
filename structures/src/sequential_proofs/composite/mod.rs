
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
