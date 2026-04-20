use path_lib::obj_at_path::OwnedObjAtPath;

use crate::{propositions::Proposition, propositions::{ParentOfAssumptions, ParentOfExplicitConclusions, paths::{AssumptionInSequentialProofStepPath, ExplicitConclusionInSequentialProofStepPath}}};

pub mod located;

pub trait InferenceRule<P: Proposition>: 'static + Clone + PartialEq {}

#[derive(Clone,PartialEq,Eq,Debug)]
/// A struct representing a single inference step within a proof
/// Every [Inference] must be an instance of a particular [InferenceRule]; The tbl_verification crate allows this to be validated.
pub struct Inference<P: Proposition,Rule:InferenceRule<P>> {
    pub inference_type: Rule,
    pub assumptions: Box<[P]>,
    pub conclusion: P
}

impl <P: Proposition, Rule: InferenceRule<P>> ParentOfAssumptions<P> for Inference<P, Rule> {
    fn get_assumption_paths(&self) -> impl IntoIterator<Item = AssumptionInSequentialProofStepPath>
        { (0..self.assumptions.len()).map(|n| AssumptionInSequentialProofStepPath::from(n)) }

    fn get_assumption(&self,path: &AssumptionInSequentialProofStepPath) -> Result<&P,()>
        { self.assumptions.get(path.0).ok_or(()) }

    fn get_assumptions<'a>(&'a self) -> impl IntoIterator<Item = &'a P> where P: 'a { &self.assumptions }
    
    fn into_located_assumptions_owned(self) -> impl IntoIterator<Item = OwnedObjAtPath<P,AssumptionInSequentialProofStepPath>> where Self:Sized {
        self.assumptions.into_iter()
            .enumerate()
            .map(|(id,conclusion)| OwnedObjAtPath{obj: conclusion, path: AssumptionInSequentialProofStepPath::from(id)})
    }
}

impl <P: Proposition, Rule: InferenceRule<P>> ParentOfExplicitConclusions<P> for Inference<P, Rule> {
    fn get_explicit_conclusion_paths(&self) -> impl IntoIterator<Item = ExplicitConclusionInSequentialProofStepPath>
        { [ExplicitConclusionInSequentialProofStepPath(0)] }

    fn get_explicit_conclusion(&self,path: &ExplicitConclusionInSequentialProofStepPath) -> Result<&P,()>
        { if path.0 == 0 { Ok(&self.conclusion) } else { Err(()) } }

    fn get_explicit_conclusions<'a>(&'a self) -> impl IntoIterator<Item = &'a P> where P: 'a { [&self.conclusion] }

    fn into_located_explicit_conclusions_owned(self) -> impl IntoIterator<Item = OwnedObjAtPath<P,ExplicitConclusionInSequentialProofStepPath>> where Self:Sized
        { [OwnedObjAtPath{obj: self.conclusion, path: ExplicitConclusionInSequentialProofStepPath(0)}] }
}

// Feature: Verification
pub mod verifiable {
    use crate::{proofs::{errors::ValidatableInferenceRule, inferences::Inference}, propositions::Proposition};

    pub struct ValidInference<P: Proposition, Rule: ValidatableInferenceRule<P>>(Inference<P,Rule>);
    impl <P: Proposition, Rule: ValidatableInferenceRule<P>> ValidInference<P,Rule> {
        pub fn inner(&self) -> &Inference<P,Rule> { &self.0 }
        /// Mark an inference as valid without actually performing a validity check.
        /// This should ONLY be used when the inference is produced by a process which guarantees validity
        pub fn unchecked(inner: Inference<P,Rule>) -> Self { Self(inner) }
    }
    impl <P: Proposition, E, Rule: ValidatableInferenceRule<P,Err=E>> TryFrom<Inference<P,Rule>> for ValidInference<P,Rule> {
        type Error = E;

        fn try_from(value: Inference<P,Rule>) -> Result<Self, Self::Error>
            { Rule::validate(&value).map(|_| Self(value)) }
    }
    impl <P: Proposition, E, Rule: ValidatableInferenceRule<P,Err=E>> Into<Inference<P,Rule>> for ValidInference<P,Rule> {
        fn into(self) -> Inference<P,Rule> { self.0 }
    }
}
