use path_lib::obj_at_path::{ObjAtPath, OwnedObjAtPath};

use crate::structures::{propositions::Proposition, inferences::{Inference, InferenceRule}, sequential_proofs::{SequentialProof, composite::CompositeSequentialProof}};

pub enum ProofAtPathEnum<'a,P: Proposition, Path,Rule: InferenceRule<P>> {
    Inference(ObjAtPath<'a,Inference<P,Rule>,Path>),
    Composite(ObjAtPath<'a,CompositeSequentialProof<P,Rule>,Path>)
}
impl <'a,P: Proposition,Path,Rule:InferenceRule<P>> ProofAtPathEnum<'a,P,Path,Rule> {
    pub fn path(&self) -> &Path { match self {
        ProofAtPathEnum::Inference(obj_at_path) => &obj_at_path.path,
        ProofAtPathEnum::Composite(obj_at_path) => &obj_at_path.path,
    }}
}
impl <'a,P: Proposition, Path,Rule:InferenceRule<P>> From<ObjAtPath<'a,SequentialProof<P,Rule>,Path>> for ProofAtPathEnum<'a,P,Path,Rule> {
    fn from(value: ObjAtPath<'a,SequentialProof<P,Rule>,Path>) -> Self { match value.obj {
        SequentialProof::Inference(inference) => Self::Inference(ObjAtPath { obj: inference, path: value.path }),
        SequentialProof::Composite(composite) => Self::Composite(ObjAtPath { obj: &composite, path: value.path }),
    }}
}
// impl <'a,Path,Rule:InferenceRule> Into<ObjAtPath<'a,Proof<Rule>,Path>> for ProofAtPathEnum<'a,Path,Rule> {
//     fn into(self) -> ObjAtPath<'a,Proof<Rule>, Path> { match self {
//         Self::Inference(inner) => ObjAtPath { obj: Proof::Inference(inner.obj), path: inner.path },
//         Self::Composite(inner) => ObjAtPath { obj: Proof::Composite(inner.obj), path: inner.path },
//     }}
// }

pub enum OwnedProofAtPathEnum<P:Proposition,Path,Rule:InferenceRule<P>> {
    Inference(OwnedObjAtPath<Inference<P,Rule>,Path>),
    Composite(OwnedObjAtPath<CompositeSequentialProof<P,Rule>,Path>)
}
impl <P:Proposition, Path,Rule:InferenceRule<P>> From<OwnedObjAtPath<SequentialProof<P,Rule>,Path>> for OwnedProofAtPathEnum<P,Path,Rule> {
    fn from(value: OwnedObjAtPath<SequentialProof<P,Rule>,Path>) -> Self { match value.obj {
        SequentialProof::Inference(inference) => Self::Inference(OwnedObjAtPath { obj: inference, path: value.path }),
        SequentialProof::Composite(composite) => Self::Composite(OwnedObjAtPath { obj: composite, path: value.path }),
    }}
}
impl <P: Proposition, Path,Rule:InferenceRule<P>> Into<OwnedObjAtPath<SequentialProof<P,Rule>,Path>> for OwnedProofAtPathEnum<P,Path,Rule> {
    fn into(self) -> OwnedObjAtPath<SequentialProof<P,Rule>, Path> { match self {
        Self::Inference(inner) => OwnedObjAtPath { obj: SequentialProof::Inference(inner.obj), path: inner.path },
        Self::Composite(inner) => OwnedObjAtPath { obj: SequentialProof::Composite(inner.obj), path: inner.path },
    }}
}
