use crate::{atoms::AtomId, expressions::Expression};

/// An object which uniquely identifies a given [Expression], while being structured differently
#[derive(Clone,PartialEq,Eq,Hash,Debug)]
pub struct ExprSignatures {
    structure: ExprStructureSignature,
    contents: ExprContentsSignature
}

impl ExprSignatures {
    pub fn get_structure(&self) -> &ExprStructureSignature {&self.structure }
    pub fn get_atoms(&self) -> &ExprContentsSignature { &self.contents }
}

impl Into<Expression> for ExprSignatures {
    fn into(self) -> Expression
        { self.structure.into_expression_inner(&self.contents, &mut 0) }
}

impl From<Expression> for ExprSignatures {
    fn from(expr: Expression) -> Self {
        match expr {
            Expression::Atomic(atom_id) => ExprSignatures {
                structure: ExprStructureSignature::Atom,
                contents: ExprContentsSignature(vec![atom_id])
            }, Expression::Tuple(expressions) => {
                let (structures, contents): (Vec<ExprStructureSignature>, Vec<ExprContentsSignature>) = expressions
                    .into_iter()
                    .map(|e| { let s: ExprSignatures = e.into(); (s.structure, s.contents)})
                    .unzip();
                ExprSignatures {
                    structure: ExprStructureSignature::Tuple(structures),
                    contents: ExprContentsSignature(contents
                        .into_iter()
                        .map(|x| x.0)
                        .flatten()
                        .collect()
                    ),
                }
            }
        }
    }
}


#[derive(Clone,PartialEq,Eq,Hash,Debug)]
pub enum ExprStructureSignature {
    Atom,
    Tuple(Vec<ExprStructureSignature>)
}

impl ExprStructureSignature {
    fn into_expression_inner(self, contents: &ExprContentsSignature, atom_ix: &mut usize) -> Expression {
        match self {
            ExprStructureSignature::Atom => { 
                let e = Expression::Atomic(contents.0[*atom_ix]);
                *atom_ix += 1;
                e
            }, ExprStructureSignature::Tuple(signatures) => Expression::Tuple(signatures
                .into_iter()
                .map(|signature| signature.into_expression_inner(contents, atom_ix))
                .collect()
            )
        }
    }
}

#[derive(Clone,PartialEq,Eq,Hash,Debug)]
pub struct ExprContentsSignature(pub Vec<AtomId>);
