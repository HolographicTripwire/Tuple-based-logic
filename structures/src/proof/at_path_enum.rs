use path_lib::obj_at_path::{ObjAtPath, OwnedObjAtPath};

use crate::proof::{Proof, composite::CompositeProof, inference::{Inference, InferenceRule}};

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
