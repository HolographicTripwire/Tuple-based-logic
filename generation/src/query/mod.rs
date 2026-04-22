use std::collections::{HashMap, HashSet};

use tbl_expressions::assigned::Expression;

struct QueryAssignment {
    x: Vec<Expression>
}
type QueryAssignmentSet = HashSet<QueryAssignment>;

struct QueryResult<AxiomSource> {
    axiom_sources: HashMap<AxiomSource,QueryAssignmentSet>,
    inference_sources: HashMap<QueryConjunction<AxiomSource>,QueryAssignmentSet>,
}

struct QueryConjunction<AxiomSource> {
    conjoined: HashMap<QueryResult<AxiomSource>,QueryAssignmentSet>
}
