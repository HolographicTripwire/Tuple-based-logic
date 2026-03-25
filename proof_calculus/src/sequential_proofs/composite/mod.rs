use path_lib::obj_at_path::{ObjAtPath, OwnedObjAtPath};

use crate::{expressions::Proposition, sequential_proofs::{AssumptionInProofStepPath, ExplicitConclusionInProofStepPath, ParentOfAssumptions, ParentOfExplicitConclusions, ParentOfSubproofs, Proof, ProofInProofPath, at_path_enum::ProofAtPathEnum, immediate::{ImmediateProofInProofPath, ParentOfImmediateSubproof}, inference::{TblInference, InferenceRule}}};

#[derive(Clone,PartialEq,Eq,Debug)]
pub struct CompositeSequentialProof<Proposition, Rule: InferenceRule<Proposition> {
    pub assumptions: Box<[Proposition]>,
    pub subproofs: Box<[Proof<Rule>]>,
    pub explicit_conclusions: Box<[Proposition]>,
}
impl <Rule: InferenceRule> ParentOfAssumptions for CompositeSequentialProof<Rule> {
    fn get_assumption_paths(&self) -> impl IntoIterator<Item = AssumptionInProofStepPath>
        { (0..self.assumptions.len()).map(|n| AssumptionInProofStepPath(n)) }

    fn get_assumption(&self,path: &AssumptionInProofStepPath) -> Result< &Proposition,()> {
        self.assumptions.get(path.0).ok_or(())
    }

    fn get_assumptions(&self) -> impl IntoIterator<Item = &Proposition> { &self.assumptions }
}
impl <Rule: InferenceRule> ParentOfExplicitConclusions for CompositeSequentialProof<Rule> {
    fn get_explicit_conclusion_paths(&self) -> impl IntoIterator<Item = ExplicitConclusionInProofStepPath> 
        { (0..self.explicit_conclusions.len()).map(|n| ExplicitConclusionInProofStepPath(n)) }

    fn get_explicit_conclusion(&self,path: &ExplicitConclusionInProofStepPath) -> Result< &Proposition,()>  {
        self.explicit_conclusions.get(path.0).ok_or(())
    }

    fn get_explicit_conclusions(&self) -> impl IntoIterator<Item = &Proposition> { &self.explicit_conclusions }
}

impl <Rule:InferenceRule> ParentOfImmediateSubproof<Rule> for CompositeSequentialProof<Rule> {
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

impl <Rule: InferenceRule> CompositeSequentialProof<Rule> {
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
impl <Rule: InferenceRule> ParentOfSubproofs<Rule> for CompositeSequentialProof<Rule> {
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

pub type InferenceInProof<'a, Rule> = ObjAtPath<'a,TblInference<Rule>,ProofInProofPath>;
pub type OwnedInferenceInProof<Rule> = OwnedObjAtPath<TblInference<Rule>,ProofInProofPath>;
