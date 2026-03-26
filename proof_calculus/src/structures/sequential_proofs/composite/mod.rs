use path_lib::obj_at_path::{ObjAtPath, OwnedObjAtPath};

use crate::structures::{Proposition, inferences::{Inference, InferenceRule}, propositions::paths::ExplicitConclusionInProofStepPath, sequential_proofs::{AssumptionInProofStepPath, ParentOfAssumptions, ParentOfExplicitConclusions, SequentialProof, at_path_enum::ProofAtPathEnum, subproofs::{ParentOfSubproofs, ProofInProofPath, immediate::{ImmediateProofInProofPath, ParentOfImmediateSubproofs}}}};

#[derive(Clone,PartialEq,Eq,Debug)]
pub struct CompositeSequentialProof<P: Proposition, Rule: InferenceRule<P>> {
    pub assumptions: Box<[P]>,
    pub subproofs: Box<[SequentialProof<P,Rule>]>,
    pub explicit_conclusions: Box<[P]>,
}
impl <P: Proposition, Rule: InferenceRule<P>> ParentOfAssumptions<P> for CompositeSequentialProof<P,Rule> {
    fn get_assumption_paths(&self) -> impl IntoIterator<Item = AssumptionInProofStepPath>
        { (0..self.assumptions.len()).map(|n| AssumptionInProofStepPath(n)) }

    fn get_assumption(&self,path: &AssumptionInProofStepPath) -> Result< &P,()> {
        self.assumptions.get(path.0).ok_or(())
    }

    fn get_assumptions<'a>(&'a self) -> impl IntoIterator<Item = &'a P> where P: 'a { &self.assumptions }
}
impl <P: Proposition, Rule: InferenceRule<P>> ParentOfExplicitConclusions<P> for CompositeSequentialProof<P,Rule> {
    fn get_explicit_conclusion_paths(&self) -> impl IntoIterator<Item = ExplicitConclusionInProofStepPath> 
        { (0..self.explicit_conclusions.len()).map(|n| ExplicitConclusionInProofStepPath(n)) }

    fn get_explicit_conclusion(&self,path: &ExplicitConclusionInProofStepPath) -> Result< &P,()>  {
        self.explicit_conclusions.get(path.0).ok_or(())
    }

    fn get_explicit_conclusions<'a>(&'a self) -> impl IntoIterator<Item = &'a P> where P: 'a { &self.explicit_conclusions }
}

impl <P: Proposition, Rule:InferenceRule<P>> ParentOfImmediateSubproofs<P,Rule> for CompositeSequentialProof<P,Rule> {
    fn get_immediate_subproof_paths(&self) -> impl IntoIterator<Item = ImmediateProofInProofPath>
        { (0..self.subproofs.len()).map(|ix| ix.into()) }
    fn get_immediate_subproof(&self,path: &ImmediateProofInProofPath) -> Result< &SequentialProof<P,Rule> ,()> 
        { self.subproofs.get(path.0).ok_or(()) }
    fn into_located_immediate_subproofs_owned(self) -> impl IntoIterator<Item = OwnedObjAtPath<SequentialProof<P,Rule>,ImmediateProofInProofPath>> where SequentialProof<P,Rule>: Clone, Self: Sized {
        self.subproofs.into_iter()
            .enumerate()
            .map(|(id,proof)| OwnedObjAtPath{obj: proof, path: id.into()})
    }
}

impl <P:Proposition, Rule: InferenceRule<P>> CompositeSequentialProof<P,Rule> {
    fn get_subproofs_helper(&self,path: &ProofInProofPath, index: usize) -> Result<&SequentialProof<P,Rule>,()> {
        let immediate_path = path.0.get(index).ok_or(())?;
        let inner = self.get_immediate_subproof(immediate_path)?;
        if index == path.0.len() { Ok(inner) }
        else { match inner {
            SequentialProof::Inference(_) => Err(()),
            SequentialProof::Composite(composite) => composite.get_subproofs_helper(path, index+1),
        }}
    }
}
impl <P: Proposition, Rule: InferenceRule<P>> ParentOfSubproofs<P,Rule> for CompositeSequentialProof<P,Rule> {
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

    fn get_subproof(&self,path: &ProofInProofPath) -> Result<&SequentialProof<P,Rule>,()>
        { self.get_subproofs_helper(path, 0) }
}

pub type InferenceInProof<'a, P,Rule> = ObjAtPath<'a,Inference<P,Rule>,ProofInProofPath>;
pub type OwnedInferenceInProof<P,Rule> = OwnedObjAtPath<Inference<P,Rule>,ProofInProofPath>;
