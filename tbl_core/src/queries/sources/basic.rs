use tbl_generation::{PropositionSource, ProvabilityMetric};

use crate::queries::CorePropositionQuery;

struct BasicCorePropositionSource {

}

impl ProvabilityMetric for () {}

impl PropositionSource<CorePropositionQuery,()> for BasicCorePropositionSource {
    fn get_propositions(query: Q) -> impl IntoIterator<Item=tbl_structures::expressions::Proposition> {
        todo!()
    }
    
    fn get_provability(&self, prop: tbl_structures::expressions::Proposition) -> &() {
        todo!()
    }
}
