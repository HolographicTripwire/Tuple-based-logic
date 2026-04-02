use path_lib::obj_at_path::{ObjAtPath, OwnedObjAtPath};

use crate::structures::propositions::{Proposition, paths::{AssumptionInSequentialProofStepPath, ExplicitConclusionInSequentialProofStepPath, PropositionInSequentialProofStepPath}};

// type LocatedProposition<'a,P: Proposition, Path> = ObjAtPath<'a,P,Path>;
// type OwnedLocatedProposition<P: Proposition, Path> = OwnedObjAtPath<P,Path>;

pub type AssumptionInSequentialProofStep<'a,P: Proposition> = ObjAtPath<'a,P,AssumptionInSequentialProofStepPath>;
pub type OwnedAssumptionInSequentialProofStep<P: Proposition> = OwnedObjAtPath<P,AssumptionInSequentialProofStepPath>;

pub type ExplicitConclusionInSequentialProofStep<'a,P: Proposition> = ObjAtPath<'a,P,ExplicitConclusionInSequentialProofStepPath>;
pub type OwnedExplicitConclusionInSequentialProofStep<P: Proposition> = OwnedObjAtPath<P,ExplicitConclusionInSequentialProofStepPath>;

pub type PropositionInSequentialProofStep<'a,P: Proposition> = ObjAtPath<'a,P,PropositionInSequentialProofStepPath>;
pub type OwnedPropositionInSequentialProofStep<P: Proposition> = OwnedObjAtPath<P,PropositionInSequentialProofStepPath>;
