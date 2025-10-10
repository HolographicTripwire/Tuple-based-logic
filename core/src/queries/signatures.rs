use crate::queries::CorePropositionQuery;

pub struct CorePropositionQuerySignatures {
    structure: CorePropositionQueryStructureSignature,
    contents: CorePropositionQueryContentsSignature,
    variables: CorePropositionQueryVariablesSignature
}

pub enum CorePropositionQueryStructureSignature {
    Atom,
    Variable,
    Tuple(Vec<CorePropositionQueryContentsSignature>)
}

pub struct CorePropositionQueryContentsSignature(Vec<AtomId>);

pub struct CorePropositionQueryVariablesSignature(Vec<usize>);

impl CorePropositionQuery {
    pub fn as_signatures(query: &CorePropositionQuery) -> Self {
        let mut contents = ExprContentsSignature(vec![]);
        let mut variables = ExprVariablesSignature(vec![]);
        let structure = Self::from_expression_inner(self, &mut contents, &mut variables);
        ExprSignatures { structure, contents, variables }
    }
}

impl CorePropositionQuerySignatures {
    pub fn as_query(&self) -> CorePropositionQuery {
        Self::into_expression_inner(&self.structure, &self.contents, &mut 0, &self.variables, &mut 0)
    }

    pub fn get_structure(&self) -> &CorePropositionQueryStructureSignature { &self.structure }
    pub fn get_contents(&self) -> &CorePropositionQueryContentsSignature { &self.contents }
    pub fn get_variables(&self) -> &CorePropositionQueryVariablesSignature { &self.variables }
}

impl From<CorePropositionQuery> for CorePropositionQuerySignatures {
    fn from(query: CorePropositionQuery) -> Self { Self::new(&query) }
}

impl Into<CorePropositionQuery> for CorePropositionQuerySignatures {
    fn into(self) -> CorePropositionQuery { self.as_query() }
}

impl CorePropositionQuerySignatures {
    fn from_query_inner(query: &CorePropositionQuery, contents: &mut CorePropositionQueryContentsSignature, variables: &mut CorePropositionQueryVariablesSignature) -> CorePropositionQueryStructureSignature {
        match query {
            CorePropositionQuery::Atomic(atom_id) => {
                contents.0.push(atom_id);
                ExprStructureSignature::Atom
            }, CorePropositionQuery::Variable(variable_id) => {
                variables.0.push(variable_id)
            }, CorePropositionQuery::Tuple(expressions) => {
                ExprStructureSignature::Tuple(expressions
                    .into_iter()
                    .map(|q| Self::from_query_inner(q, contents, variables))
                    .collect()
                )
            },
        }
    }

    fn into_query_inner(structure: &CorePropositionQueryStructureSignature, contents: &CorePropositionQueryContentsSignature, content_ix: &mut usize, variables: &CorePropositionQueryVariablesSignature, variable_ix: &mut usize) -> CorePropositionQuery {
        match structure {
            ExprStructureSignature::Atom => { 
                let q = CorePropositionQuery::Atomic(contents.0[*content_ix]);
                *content_ix += 1;
                q
            }, ExprStructureSignature::Variable => {
                let q = CorePropositionQuery::Variable(variables.0[*variable_ix]);
                *variable_ix += 1;
                q
            } ExprStructureSignature::Tuple(signatures) => Expression::Tuple(signatures
                .into_iter()
                .map(|structure| Self::into_expression_inner(structure, contents, content_ix, variables, variable_ix))
                .collect()
            )
        }
    }
}
