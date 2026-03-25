pub mod immediate;
mod proposition;





// impl <Rule:InferenceRule> HasChildren<AtomicProofInProofPath,Proof<Rule>> for Proof<Rule> {
//     fn valid_primitive_paths(&self) -> Vec<AtomicProofInProofPath> {
//         let max = if let Proof::Composite(composite) = self
//             { composite.subproofs.len() } else { 0 };
//         (0..max).map(|ix| ix.into()).collect()
//     }
    
//     fn get_child(&self, path: &AtomicProofInProofPath) -> Result<&Proof<Rule>,()> {
//         if let Proof::Composite(composite) = self
//             { composite.get_child(path) }
//         else { Err(()) }
//     }
    
//     fn get_child_owned(&self, path: &AtomicProofInProofPath) -> Result<Proof<Rule>,()> where Proof<Rule>: Clone {
//         if let Proof::Composite(composite) = self
//             { composite.get_child_owned(path) }
//         else { Err(()) }
//     }
    
//     fn into_located_children_owned(self) -> impl IntoIterator<Item = OwnedObjAtPath<Proof<Rule>,AtomicProofInProofPath>> where Proof<Rule>: Clone, Self: Sized {
//         match self {
//             Proof::Inference(_) => vec![],
//             Proof::Composite(composite_proof) => <CompositeProof<Rule> as HasChildren<AtomicProofInProofPath,Proof<Rule>>>
//                 ::into_located_children_owned(composite_proof).into_iter().collect(),
//         }
//     }
// }
