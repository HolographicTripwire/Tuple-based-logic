use crate::expressions::assigned::{TblExpression, atomic::AtomicTblExpression, compound::CompoundTblExpression};

/// An object which uniquely identifies a given [Expression], while being structured differently
#[derive(Clone,PartialEq,Eq,Hash,Debug)]
pub struct ExprSignature {
    structure: ExprStructureSignature,
    contents: ExprContentsSignature
}

#[derive(Clone,PartialEq,Eq,Hash,Debug)]
pub enum ExprStructureSignature {
    Atomic,
    Compound(CompoundExprStructureSignature)
}
#[derive(Clone,PartialEq,Eq,Hash,Debug)]
pub struct CompoundExprStructureSignature(pub Box<[ExprStructureSignature]>);

#[derive(Clone,PartialEq,Eq,Hash,Debug)]
pub struct ExprContentsSignature(pub Box<[AtomicTblExpression]>);

impl ExprSignature {
    pub fn get_structure(&self) -> &ExprStructureSignature {&self.structure }
    pub fn get_atoms(&self) -> &ExprContentsSignature { &self.contents }
}

impl <C: CompoundTblExpression> Into<ExprSignature> for TblExpression<C>
    { fn into(self) -> ExprSignature {
        let mut contents = vec![];
        let structure = ExprSignature::from_expression_inner(self, &mut contents);
        ExprSignature { structure, contents: ExprContentsSignature(contents.into()) }
    } }
impl <C: CompoundTblExpression> From<ExprSignature> for TblExpression<C> where TblExpression<C>: From<Box<[TblExpression<C>]>>
    { fn from(other: ExprSignature) -> TblExpression<C> { ExprSignature::into_expression_inner(other.structure, &other.contents, &mut 0) } }

impl ExprSignature {
    fn from_expression_inner<C: CompoundTblExpression>(expr: TblExpression<C>, contents: &mut Vec<AtomicTblExpression>) -> ExprStructureSignature {
        match expr {
            TblExpression::Atomic(atom_id) => {
                contents.push(atom_id);
                ExprStructureSignature::Atomic
            }, TblExpression::Compound(expressions) => expressions
                .into_immediate_subexpressions_owned()
                .into_iter()
                .map(|expr| Self::from_expression_inner(expr, contents))
                .collect_vec()
                .into()
        }
    }

    fn into_expression_inner<C: CompoundTblExpression>(structure: ExprStructureSignature, contents: &ExprContentsSignature, content_ix: &mut usize) -> TblExpression<C> where TblExpression<C>: From<Box<[TblExpression<C>]>> {
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
    use crate::structures::expression_signatures::{CompoundExprStructureSignature, ExprStructureSignature};

    impl From<()> for ExprStructureSignature {
        fn from(_value: ()) -> Self { Self::Atomic }
    }
    impl From<Vec<ExprStructureSignature>> for CompoundExprStructureSignature {
        fn from(value: Vec<ExprStructureSignature>) -> Self { Self(value.into()) }
    }
    impl From<CompoundExprStructureSignature> for ExprStructureSignature {
        fn from(value: CompoundExprStructureSignature) -> Self { Self::Compound(value) }
    }
    impl From<Vec<ExprStructureSignature>> for ExprStructureSignature {
        fn from(value: Vec<ExprStructureSignature>) -> Self { Self::Compound(value.into()) }
    }
}
