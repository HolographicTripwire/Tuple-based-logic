use crate::{atoms::AtomId, expressions::Expression};

/// An object which uniquely identifies a given [Expression], while being structured differently
#[derive(Clone,PartialEq,Eq,Hash,Debug)]
pub struct ExprSignatures {
    structure: ExprStructureSignature,
    contents: ExprContentsSignature
}

#[derive(Clone,PartialEq,Eq,Hash,Debug)]
pub enum ExprStructureSignature {
    Atom,
    Tuple(Vec<ExprStructureSignature>)
}

#[derive(Clone,PartialEq,Eq,Hash,Debug)]
pub struct ExprContentsSignature(pub Vec<AtomId>);

impl ExprSignatures {
    pub fn new(expr: &Expression) -> Self {
        let mut contents = ExprContentsSignature(vec![]);
        let structure = Self::from_expression_inner(expr, &mut contents);
        ExprSignatures { structure, contents }
    }

    pub fn as_expression(&self) -> Expression {
        Self::into_expression_inner(&self.structure, &self.contents, &mut 0)
    }

    pub fn get_structure(&self) -> &ExprStructureSignature {&self.structure }
    pub fn get_atoms(&self) -> &ExprContentsSignature { &self.contents }
}

impl From<Expression> for ExprSignatures
    { fn from(expr: Expression) -> Self { Self::new(&expr) } }
impl Into<Expression> for ExprSignatures
    { fn into(self) -> Expression { Self::as_expression(&self) } }

impl ExprSignatures {
    fn from_expression_inner(expr: &Expression, contents: &mut ExprContentsSignature) -> ExprStructureSignature {
        match expr {
            Expression::Atomic(atom_id) => {
                contents.0.push(*atom_id);
                ExprStructureSignature::Atom
            }, Expression::Tuple(expressions) => {
                ExprStructureSignature::Tuple(expressions
                    .into_iter()
                    .map(|expr| Self::from_expression_inner(expr, contents))
                    .collect()
                )
            },
        }
    }

    fn into_expression_inner(structure: &ExprStructureSignature, contents: &ExprContentsSignature, content_ix: &mut usize) -> Expression {
        match structure {
            ExprStructureSignature::Atom => { 
                let e = Expression::Atomic(contents.0[*content_ix]);
                *content_ix += 1;
                e
            }, ExprStructureSignature::Tuple(signatures) => Expression::Tuple(signatures
                .into_iter()
                .map(|structure| Self::into_expression_inner(structure, contents, content_ix))
                .collect()
            )
        }
    }
}
