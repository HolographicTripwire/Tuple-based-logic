use std::collections::HashSet;

use tbl_structures::inference::InferenceRule;

use crate::PropositionQuery;

pub trait QueryableInferenceRule<Q: PropositionQuery>: InferenceRule {
    fn query(query: Q) -> HashSet<Q>;
}
