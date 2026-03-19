use path_lib::obj_at_path::{ObjAtPath, OwnedObjAtPath};
use path_lib_proc_macros::generate_parent_of_children_trait;

use crate::proof::{AtomicProofInProofPath, Proof, inference::InferenceRule};

pub type ImmediateProofInProof<'a,Rule> = ObjAtPath<'a,Proof<Rule>,AtomicProofInProofPath>;
pub type OwnedImmediateProofInProof<Rule> = OwnedObjAtPath<Proof<Rule>,AtomicProofInProofPath>;

generate_parent_of_children_trait!{
    (Proof<Rule> where Rule: InferenceRule), AtomicProofInProofPath,
    "immediate_subproof", "immediate_subproofs", "ImmediateSubproof"
}
