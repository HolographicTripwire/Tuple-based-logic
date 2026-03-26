use crate::expressions::{TblExpression, atomic::AtomicTblExpression};


/// An object which uniquely identifies a given [Expression], while being structured differently
#[derive(Clone,PartialEq,Eq,Hash,Debug)]
pub struct ExprSignatures {
    structure: ExprStructureSignature,
    contents: ExprContentsSignature
}

#[derive(Clone,PartialEq,Eq,Hash,Debug)]
pub enum ExprStructureSignature {
    Atomic,
    Compound(CompoundExprStructureSignature)
}
#[derive(Clone,PartialEq,Eq,Hash,Debug)]
pub struct CompoundExprStructureSignature(Vec<ExprStructureSignature>);

#[derive(Clone,PartialEq,Eq,Hash,Debug)]
pub struct ExprContentsSignature(pub Vec<AtomicTblExpression>);

impl ExprSignatures {

    pub fn get_structure(&self) -> &ExprStructureSignature {&self.structure }
    pub fn get_atoms(&self) -> &ExprContentsSignature { &self.contents }
}

impl Into<ExprSignatures> for TblExpression
    { fn into(self) -> ExprSignatures {
        let mut contents = ExprContentsSignature(vec![]);
        let structure = ExprSignatures::from_expression_inner(self, &mut contents);
        ExprSignatures { structure, contents }
    } }
impl From<ExprSignatures> for TblExpression
    { fn from(other: ExprSignatures) -> TblExpression { ExprSignatures::into_expression_inner(other.structure, &other.contents, &mut 0) } }

impl ExprSignatures {
    fn from_expression_inner(expr: TblExpression, contents: &mut ExprContentsSignature) -> ExprStructureSignature {
        match expr {
            TblExpression::Atomic(atom_id) => {
                contents.0.push(atom_id);
                ExprStructureSignature::Atomic
            }, TblExpression::Compound(expressions) => expressions.0
                .iter().cloned()
                .map(|expr| Self::from_expression_inner(expr, contents))
                .collect::<Vec<_>>()
                .into()
        }
    }

    fn into_expression_inner(structure: ExprStructureSignature, contents: &ExprContentsSignature, content_ix: &mut usize) -> TblExpression {
        match structure {
            ExprStructureSignature::Atomic => { 
                let e = TblExpression::Atomic(contents.0[*content_ix]);
                *content_ix += 1;
                e
            }, ExprStructureSignature::Compound(signatures) => signatures.0
                .into_iter()
                .map(|structure| Self::into_expression_inner(structure, contents, content_ix))
                .collect::<Box<[_]>>()
                .into()
        }
    }
}

mod from {
    use crate::expressions::{ExprStructureSignature, signatures::CompoundExprStructureSignature};

    impl From<()> for ExprStructureSignature {
        fn from(_value: ()) -> Self { Self::Atomic }
    }
    impl From<Vec<ExprStructureSignature>> for CompoundExprStructureSignature {
        fn from(value: Vec<ExprStructureSignature>) -> Self { Self(value) }
    }
    impl From<CompoundExprStructureSignature> for ExprStructureSignature {
        fn from(value: CompoundExprStructureSignature) -> Self { Self::Compound(value) }
    }
    impl From<Vec<ExprStructureSignature>> for ExprStructureSignature {
        fn from(value: Vec<ExprStructureSignature>) -> Self { Self::Compound(value.into()) }
    }
}
