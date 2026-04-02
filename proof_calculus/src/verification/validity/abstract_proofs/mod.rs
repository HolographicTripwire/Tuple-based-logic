use std::collections::{HashMap, HashSet};

use crate::{structures::propositions::Proposition, verification::validity::{ValidatableInferenceRule, inferences::ValidInference}};

pub struct ValidAbstractProof<P: Proposition, Rule: ValidatableInferenceRule<P>> {
    inferences: HashMap<P,ValidInference<P,Rule>>,
}
impl <P: Proposition, Rule: ValidatableInferenceRule<P>> ValidAbstractProof<P,Rule> {
    pub fn insert(&mut self, inference: ValidInference<P,Rule>) -> Option<ValidInference<P, Rule>>
        { self.inferences.insert(inference.inner().conclusion.clone(), inference) }
    pub fn inner_map(&self) -> &HashMap<P,ValidInference<P,Rule>> { &self.inferences }
    
    pub fn assumptions(&self) -> HashSet<&P> {
        self.inferences
            .values()
            .into_iter()
            .map(|i| &i.inner().assumptions)
            .flatten()
            .collect()
    }
    pub fn base_assumptions(&self) -> HashSet<&P> {
        self.assumptions()
            .difference(&self.conclusions())
            .map(|x| *x)
            .collect()
    }
    pub fn conclusions(&self) -> HashSet<&P> {
        self.inferences
            .keys()
            .collect()
    }
    pub fn final_conclusions(&self) -> HashSet<&P> {
        self.conclusions()
            .difference(&self.assumptions())
            .map(|x| *x)
            .collect()
    }
}
impl <P: Proposition, Rule: ValidatableInferenceRule<P>> FromIterator<ValidInference<P,Rule>> for ValidAbstractProof<P,Rule> {
    fn from_iter<T: IntoIterator<Item = ValidInference<P,Rule>>>(iter: T) -> Self {
        Self { inferences: iter.into_iter().map(|i| (i.inner().conclusion.clone(), i)).collect() }
    }
}

pub struct InferenceInValidAbstractProofPath<P: Proposition>(P);
// generate_parent_of_children_trait!{
//     (Proof<P,Rule> where P: Proposition, Rule: InferenceRule<P>), InferenceInProofPath<P>,
//     "subproof", "subproofs", "Subproofs"
// }
