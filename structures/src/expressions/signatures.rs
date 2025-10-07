use crate::{atoms::AtomId, expressions::Expression};

/// An object which uniquely identifies a given [Expression], while being structured differently
#[derive(Clone)]
pub struct ExprSignatures {
    structure: ExprStructureSignature,
    atoms: Vec<AtomId>
}

impl ExprSignatures {
    pub fn get_structure(&self) -> &ExprStructureSignature {&self.structure }
    pub fn get_atoms(&self) -> &Vec<AtomId> { &self.atoms }
}

impl Into<Expression> for ExprSignatures {
    fn into(self) -> Expression
        { self.structure.into_expression_inner(&self.atoms, &mut 0) }
}

impl From<Expression> for ExprSignatures {
    fn from(expr: Expression) -> Self {
        match expr {
            Expression::Atomic(atom_id) => ExprSignatures {
                structure: ExprStructureSignature::Atom,
                atoms: vec![atom_id]
            }, Expression::Tuple(expressions) => {
                let (structures, atoms): (Vec<ExprStructureSignature>, Vec<Vec<AtomId>>) = expressions
                    .into_iter()
                    .map(|e| { let s: ExprSignatures = e.into(); (s.structure, s.atoms)})
                    .unzip();
                ExprSignatures {
                    structure: ExprStructureSignature::Tuple(structures),
                    atoms: atoms.into_iter().flatten().collect(),
                    
                }
            }
        }
    }
}


#[derive(Clone)]
pub enum ExprStructureSignature {
    Atom,
    Tuple(Vec<ExprStructureSignature>)
}

impl ExprStructureSignature {
    fn into_expression_inner(self, atoms: &Vec<AtomId>, atom_ix: &mut usize) -> Expression {
        match self {
            ExprStructureSignature::Atom => { 
                let e = Expression::Atomic(atoms[*atom_ix]);
                *atom_ix += 1;
                e
            }, ExprStructureSignature::Tuple(signatures) => Expression::Tuple(signatures
                .into_iter()
                .map(|signature| signature.into_expression_inner(atoms, atom_ix))
                .collect()
            )
        }
    }
}
