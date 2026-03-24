use std::marker::PhantomData;

use crate::{inference_rules::QueryableInferenceRule, promise::{ProofGenerationError, ProofGenerator, ProofPromise}};

mod query;
mod promise;
mod inference_rules;

/*
struct QueryBase<Q: PropositionQuery, R: QueryableInferenceRule<Q>> {
    sources: Vec<Box<dyn PropositionSource<Q>>>,
    generators: Vec<Box<dyn ProofGenerator<R>>>
} */

pub trait PropositionQuery: From<TblProposition> {
    
}

pub trait ProvabilityMetric {}

pub trait PropositionSource<Q: PropositionQuery, M: ProvabilityMetric> {
    fn get_propositions(&self, query: Q) -> impl IntoIterator<Item=&TblProposition>;
    fn get_provability(&self, query: Q) -> &M;
}
