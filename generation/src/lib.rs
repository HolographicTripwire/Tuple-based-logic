use std::marker::PhantomData;

use tbl_structures::{expressions::Proposition, inference::InferenceRule};

use crate::{inference_rules::QueryableInferenceRule, promise::{ProofGenerationError, ProofGenerator, ProofPromise}};

mod promise;
mod inference_rules;

/*
struct QueryBase<Q: PropositionQuery, R: QueryableInferenceRule<Q>> {
    sources: Vec<Box<dyn PropositionSource<Q>>>,
    generators: Vec<Box<dyn ProofGenerator<R>>>
} */

pub trait PropositionQuery: From<Proposition> {

}

pub trait ProvabilityMetric {}

pub trait PropositionSource<Q: PropositionQuery, M: ProvabilityMetric> {
    fn get_propositions(&self, query: Q) -> impl IntoIterator<Item=&Proposition>;
    fn get_provability(&self, query: Q) -> &M;
}
