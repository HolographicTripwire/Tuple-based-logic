pub mod inference;
pub mod composite;
mod step;
mod in_proof;
pub mod error;

use path_lib::obj_at_path::{ObjAtPath, OwnedObjAtPath};
use path_lib_proc_macros::generate_parent_of_children_trait;
pub use in_proof::*;

use crate::{expressions::Proposition, proof::{composite::CompositeProof, inference::{Inference, InferenceRule}}};

generate_parent_of_children_trait!{
    (Proposition), AssumptionInProofStepPath,
    "assumption", "assumptions", "Assumptions"
}
generate_parent_of_children_trait!{
    (Proposition), ExplicitConclusionInProofStepPath,
    "explicit_conclusion", "explicit_conclusions", "ExplicitConclusions"
}
pub trait ProofStep<Rule: InferenceRule>: ParentOfAssumptions + ParentOfExplicitConclusions {}

/// This struct represents a step within a larger proof
#[derive(Clone,PartialEq,Eq,Debug)]
pub enum Proof<Rule: InferenceRule> {
    Inference(Inference<Rule>), // A single inference step
    Composite(CompositeProof<Rule>) // A composite proof made of further subproofs
}

pub enum ProofAtPathEnum<'a,Path,Rule: InferenceRule> {
    Inference(ObjAtPath<'a,Inference<Rule>,Path>),
    Composite(ObjAtPath<'a,CompositeProof<Rule>,Path>)
}
impl <'a,Path,Rule:InferenceRule> ProofAtPathEnum<'a,Path,Rule> {
    pub fn path(&self) -> &Path { match self {
        ProofAtPathEnum::Inference(obj_at_path) => &obj_at_path.path,
        ProofAtPathEnum::Composite(obj_at_path) => &obj_at_path.path,
    }}
}
impl <'a,Path,Rule:InferenceRule> From<ObjAtPath<'a,Proof<Rule>,Path>> for ProofAtPathEnum<'a,Path,Rule> {
    fn from(value: ObjAtPath<'a,Proof<Rule>,Path>) -> Self { match value.obj {
        Proof::Inference(inference) => Self::Inference(ObjAtPath { obj: inference, path: value.path }),
        Proof::Composite(composite) => Self::Composite(ObjAtPath { obj: &composite, path: value.path }),
    }}
}
// impl <'a,Path,Rule:InferenceRule> Into<ObjAtPath<'a,Proof<Rule>,Path>> for ProofAtPathEnum<'a,Path,Rule> {
//     fn into(self) -> ObjAtPath<'a,Proof<Rule>, Path> { match self {
//         Self::Inference(inner) => ObjAtPath { obj: Proof::Inference(inner.obj), path: inner.path },
//         Self::Composite(inner) => ObjAtPath { obj: Proof::Composite(inner.obj), path: inner.path },
//     }}
// }


pub enum OwnedProofAtPathEnum<Path,Rule:InferenceRule> {
    Inference(OwnedObjAtPath<Inference<Rule>,Path>),
    Composite(OwnedObjAtPath<CompositeProof<Rule>,Path>)
}
impl <Path,Rule:InferenceRule> From<OwnedObjAtPath<Proof<Rule>,Path>> for OwnedProofAtPathEnum<Path,Rule> {
    fn from(value: OwnedObjAtPath<Proof<Rule>,Path>) -> Self { match value.obj {
        Proof::Inference(inference) => Self::Inference(OwnedObjAtPath { obj: inference, path: value.path }),
        Proof::Composite(composite) => Self::Composite(OwnedObjAtPath { obj: composite, path: value.path }),
    }}
}
impl <Path,Rule:InferenceRule> Into<OwnedObjAtPath<Proof<Rule>,Path>> for OwnedProofAtPathEnum<Path,Rule> {
    fn into(self) -> OwnedObjAtPath<Proof<Rule>, Path> { match self {
        Self::Inference(inner) => OwnedObjAtPath { obj: Proof::Inference(inner.obj), path: inner.path },
        Self::Composite(inner) => OwnedObjAtPath { obj: Proof::Composite(inner.obj), path: inner.path },
    }}
}

impl <Rule: InferenceRule> ParentOfAssumptions for Proof<Rule> {
    fn get_assumption_paths(&self) -> impl IntoIterator<Item = AssumptionInProofStepPath>  { match self {
        Proof::Inference(inference) => inference.get_assumption_paths().into_iter().collect::<Vec<_>>(),
        Proof::Composite(composite) => composite.get_assumption_paths().into_iter().collect(),
    }}
    
    fn get_assumption(&self,path: &AssumptionInProofStepPath) -> Result< &Proposition,()>  { match self {
        Proof::Inference(inference) => inference.get_assumption(path),
        Proof::Composite(composite_proof) => composite_proof.get_assumption(path),
    }}
}
impl <Rule: InferenceRule> ParentOfExplicitConclusions for Proof<Rule> {
    fn get_explicit_conclusion_paths(&self) -> impl IntoIterator<Item = ExplicitConclusionInProofStepPath> {match self {
        Proof::Inference(inference) => inference.get_explicit_conclusion_paths().into_iter().collect::<Vec<_>>(),
        Proof::Composite(composite) => composite.get_explicit_conclusion_paths().into_iter().collect(),
    }}
    
    fn get_explicit_conclusion(&self,path: &ExplicitConclusionInProofStepPath) -> Result< &Proposition,()>  {match self {
        Proof::Inference(inference) => inference.get_explicit_conclusion(path),
        Proof::Composite(composite_proof) => composite_proof.get_explicit_conclusion(path),
    }}
}
impl <Rule: InferenceRule> ProofStep<Rule> for Proof<Rule> {}

// impl <Rule:InferenceRule> HasChildren<PropositionInProofStepPath,Proposition> for Proof<Rule> {
//     fn valid_primitive_paths(&self) -> Vec<PropositionInProofStepPath> { valid_primitive_paths_inner(
//         self,
//         self.explicit_conclusion_paths().into_iter().count()
//     )}

//     fn get_child(&self, path: &PropositionInProofStepPath) -> Result<&Proposition,()> { get_child_inner(self,path) }
//     fn get_child_owned(&self, path: &PropositionInProofStepPath) -> Result<Proposition,()> where Proposition: Clone
//         { get_child_inner(self, path).cloned() }
        
//     fn into_located_children_owned(self) -> impl IntoIterator<Item = OwnedObjAtPath<Proposition,PropositionInProofStepPath>> where Proposition: Clone, Self: Sized {
//         match self {
//             Proof::Inference(_) => vec![],
//             Proof::Composite(composite_proof) => <CompositeProof<Rule> as HasChildren<PropositionInProofStepPath,Proposition>>
//                 ::into_located_children_owned(composite_proof)
//                 .into_iter().collect()
//         }
//     }
// }

#[cfg(test)]
mod tests {
    use crate::proof::in_proof::ProofInProofPath;    

    #[test]
    fn test_getters() {
        let step = ProofInProofPath::default();
        assert_eq!(step.0, vec![])
    }
}
