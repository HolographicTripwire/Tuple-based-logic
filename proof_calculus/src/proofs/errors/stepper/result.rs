use crate::{propositions::assigned::Proposition, proofs::errors::OwnedProofValidityErrorAtPath};

pub struct ProofValidityStepResultWrapper<P:Proposition,IE:Clone,ParentPath,JoinedPath> {
    pub is_finished: bool,
    pub next_result: ProofValidityStepResult<P,IE,ParentPath,JoinedPath>
}

pub type ProofValidityStepResult<P,IE,ParentPath,JoinedPath> = Result<(),ProofValidityStepErr<P,IE,ParentPath,JoinedPath>>;
#[derive(Clone,PartialEq,Eq,Debug)]
pub enum ProofValidityStepErr<P:Proposition,IE:Clone,ParentPath,JoinedPath> {
    InParent(OwnedProofValidityErrorAtPath<P,IE,ParentPath>),
    InChild(OwnedProofValidityErrorAtPath<P,IE,JoinedPath>),
}
impl <P:Proposition,IE:Clone,ParentPath,JoinedPath> ProofValidityStepErr<P,IE,ParentPath,JoinedPath> {
    pub fn replace_path<NewParentPath,NewJoinedPath>(self, parent_replace: impl Fn(ParentPath) -> NewParentPath, joined_replace: impl Fn(JoinedPath) -> NewJoinedPath) -> ProofValidityStepErr<P,IE,NewParentPath,NewJoinedPath> {
        match self {
            ProofValidityStepErr::InParent(parent) => ProofValidityStepErr::InParent(parent.replace_path(parent_replace)),
            ProofValidityStepErr::InChild(child) => ProofValidityStepErr::InChild(child.replace_path(joined_replace)),
        }
    }
}
impl <P:Proposition,IE:Clone,ParentPath,JoinedPath> ProofValidityStepResultWrapper<P,IE,ParentPath,JoinedPath> {
    pub fn unfinished_no_err() -> Self { Self {
        is_finished: false,
        next_result: Ok(())
    }}
    pub fn finished_no_err() -> Self { Self {
        is_finished: true,
        next_result: Ok(())
    }}
    pub fn unfinished_parent_err(err: OwnedProofValidityErrorAtPath<P,IE,ParentPath>) -> Self { Self {
        is_finished: false,
        next_result: Err(ProofValidityStepErr::InParent(err))
    }}
    pub fn finished_parent_err(err: OwnedProofValidityErrorAtPath<P,IE,ParentPath>) -> Self { Self {
        is_finished: true,
        next_result: Err(ProofValidityStepErr::InParent(err))
    }}
    pub fn unfinished_child_err(err: OwnedProofValidityErrorAtPath<P,IE,JoinedPath>) -> Self { Self {
        is_finished: false,
        next_result: Err(ProofValidityStepErr::InChild(err))
    }}
    pub fn finished_child_err(err: OwnedProofValidityErrorAtPath<P,IE,JoinedPath>) -> Self { Self {
        is_finished: true,
        next_result: Err(ProofValidityStepErr::InChild(err))
    }}
}
