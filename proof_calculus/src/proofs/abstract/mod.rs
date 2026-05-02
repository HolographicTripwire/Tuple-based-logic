use std::collections::{HashMap, HashSet};

use path_lib_proc_macros::generate_parent_of_children_trait;

use crate::{proofs::inferences::{Inference, InferenceRule}, propositions::types::assigned::Proposition};

pub struct AbstractProof<P: Proposition, Rule: InferenceRule<P>> {
    inferences: HashMap<P,Inference<P,Rule>>,
}
impl <P: Proposition, Rule: InferenceRule<P>> AbstractProof<P,Rule> {
    pub fn insert(&mut self, inference: Inference<P,Rule>) -> Option<Inference<P, Rule>>
        { self.inferences.insert(inference.conclusion.clone(), inference) }
    pub fn inner_map(&self) -> &HashMap<P,Inference<P,Rule>> { &self.inferences }
    
    pub fn assumptions(&self) -> HashSet<&P> {
        self.inferences
            .values()
            .into_iter()
            .map(|i| &i.assumptions)
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
impl <P: Proposition, Rule: InferenceRule<P>> FromIterator<Inference<P,Rule>> for AbstractProof<P,Rule> {
    fn from_iter<T: IntoIterator<Item = Inference<P,Rule>>>(iter: T) -> Self {
        Self { inferences: iter.into_iter().map(|i| (i.conclusion.clone(), i)).collect() }
    }
}

#[derive(Clone,PartialEq,Eq,Hash)]
pub struct InferenceInAbstractProofPath<P: Proposition>(pub P);
generate_parent_of_children_trait!{
    Inference<P,Rule>, InferenceInAbstractProofPath<P>, (P: Proposition, Rule: InferenceRule<P>),
    "inference", "inferences", "Inferences"
}

impl <P: Proposition, Rule: InferenceRule<P>> ParentOfInferences<P,Rule> for AbstractProof<P,Rule> {
    fn get_inference_paths(&self) -> impl IntoIterator<Item = InferenceInAbstractProofPath<P>> 
        { self.inferences.keys().map(|x| InferenceInAbstractProofPath(x.clone())) }
    fn get_inference(&self,path: &InferenceInAbstractProofPath<P>) -> Result< &Inference<P,Rule> ,()>
        { self.inferences.get(&path.0).ok_or(()) }
}

// Feature: verification
pub mod valid {
    use std::collections::{HashMap, HashSet};

    use crate::{proofs::{errors::ValidatableInferenceRule, inferences::verifiable::ValidInference}, propositions::types::assigned::Proposition};
    
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

}
