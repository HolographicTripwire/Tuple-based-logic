use crate::proof::{OwnedProofInProof, Proof, ProofInProof, ProofInProofPath, composite::{CompositeProofInProof, OwnedCompositeProofInProof}, inference::{InferenceInProof, InferenceRule, OwnedInferenceInProof}};

pub enum SplitProofInProof<'a,Rule: InferenceRule> {
    Inference(InferenceInProof<'a,Rule>),
    Composite(CompositeProofInProof<'a,Rule>)
}
impl <'a,Rule:InferenceRule> SplitProofInProof<'a,Rule > {
    pub fn path(&'a self) -> &'a ProofInProofPath { match self {
        SplitProofInProof::Inference(i) => i.path(),
        SplitProofInProof::Composite(c) => c.path(),
    }}
}
impl <'a,Rule:InferenceRule> From<ProofInProof<'a,Rule>> for SplitProofInProof<'a,Rule> {
    fn from(value: ProofInProof<'a,Rule>) -> Self {
        let (obj,path) = value.0.into_obj_and_path();
        match obj {
            Proof::Inference(inference) => Self::Inference(InferenceInProof::from_inner(inference, path)),
            Proof::Composite(composite) => Self::Composite(CompositeProofInProof::from_inner(composite, path)),
        }
    }
}

pub enum OwnedSplitProofInProof<Rule: InferenceRule> {
    Inference(OwnedInferenceInProof<Rule>),
    Composite(OwnedCompositeProofInProof<Rule>)
}
impl <Rule:InferenceRule> OwnedSplitProofInProof<Rule > {
    pub fn path(&self) -> &ProofInProofPath { match self {
        OwnedSplitProofInProof::Inference(i) => i.path(),
        OwnedSplitProofInProof::Composite(c) => c.path(),
    }}
}
impl <Rule:InferenceRule> From<OwnedProofInProof<Rule>> for OwnedSplitProofInProof<Rule> {
    fn from(value: OwnedProofInProof<Rule>) -> Self {
        let (obj,path) = value.0.into_obj_and_path();
        match obj {
            Proof::Inference(inference) => Self::Inference(OwnedInferenceInProof::from_inner(inference, path)),
            Proof::Composite(composite) => Self::Composite(OwnedCompositeProofInProof::from_inner(composite, path)),
        }
    }
}
