use crate::{structures::{propositions::Proposition, inferences::{Inference}}, verification::ValidatableInferenceRule};

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
