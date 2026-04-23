use path_lib::obj_at_path::{ObjAtPath, OwnedObjAtPath};

use crate::{expressions::unassigned::{compound::UnassignedCompoundTblExpression, subexpressions::{ParentOfUnassignedSubexpressions, immediate::ParentOfImmediateUnassignedSubexpressions}, variable::TblExpressionVariable}, expressions::assigned::{TblExpression, atomic::AtomicTblExpression, compound::{CompoundTblExpression, r#box::BoxCompoundTblExpression}, subexpressions::{TblSubexpressionInExpressionPath, immediate::ImmediateSubexpressionInExpressionPath}}};

pub mod variable;
pub mod compound;
pub mod subexpressions;
pub mod at_path_enum;
pub mod binding;
pub mod assignments;

#[derive(Clone,PartialEq,Eq,Hash,Debug)]
pub enum UnassignedTblExpression<C: UnassignedCompoundTblExpression> {
    Atomic(AtomicTblExpression),
    Compound(C),
    Variable(TblExpressionVariable)
}
pub type BoxUnassignedTblExpression = TblExpression<UnassignedBoxCompoundTblExpression>;
pub type RcUnassignedTblExpression = TblExpression<UnassignedRcCompoundTblExpression>;
pub type ArcUnassignedTblExpression = TblExpression<UnassignedArcCompoundTblExpression>;

pub type UnassignedTblExpressionAtPath<'a,C: UnassignedCompoundTblExpression, Path> = ObjAtPath<'a,UnassignedTblExpression<C>,Path>;
pub type BoxUnassignedTblExpressionAtPath<'a,Path> = UnassignedTblExpressionAtPath<'a,BoxCompoundTblExpression,Path>;
pub type RcUnassignedTblExpressionAtPath<'a,Path> = UnassignedTblExpressionAtPath<'a,RcCompoundTblExpression,Path>;
pub type ArcUnassignedTblExpressionAtPath<'a,Path> = UnassignedTblExpressionAtPath<'a,ArcCompoundTblExpression,Path>;

pub type OwnedUnassignedTblExpressionAtPath<C: UnassignedCompoundTblExpression, Path> = OwnedObjAtPath<UnassignedTblExpression<C>,Path>;
pub type OwnedBoxUnassignedTblExpressionAtPath<Path> = OwnedUnassignedTblExpressionAtPath<BoxCompoundTblExpression,Path>;
pub type OwnedRcUnassignedTblExpressionAtPath<Path> = OwnedUnassignedTblExpressionAtPath<RcCompoundTblExpression,Path>;
pub type OwnedArcUnassignedTblExpressionAtPath<Path> = OwnedUnassignedTblExpressionAtPath<ArcCompoundTblExpression,Path>;

impl <C: UnassignedCompoundTblExpression> UnassignedTblExpression<C> {
    pub fn replace(&self, to_replace: &UnassignedTblExpression<C>, replace_with: &UnassignedTblExpression<C>) -> Self {
        if self == to_replace { replace_with.clone() }
        else if let UnassignedTblExpression::Compound(compound) = self
            { UnassignedTblExpression::Compound(compound.replace(to_replace, replace_with)) }
        else { self.clone() }
    }

    pub fn is_atom(&self) -> bool { if let UnassignedTblExpression::Atomic(_) = self { true } else { false } }
    pub fn is_compound(&self) -> bool { if let UnassignedTblExpression::Compound(_) = self { true } else { false } }
    pub fn is_variable(&self) -> bool { if let UnassignedTblExpression::Variable(_) = self { true } else { false } }

    pub fn get_subexpressions_helper(&self,path: &TblSubexpressionInExpressionPath, index: usize) -> Result<&UnassignedTblExpression<C>,()> {
        let immediate_path = path.0.get(index).ok_or(())?;
        let inner = self.get_immediate_subexpression(immediate_path)?;
        if index == path.0.len() { Ok(inner) }
        else { inner.get_subexpressions_helper(path, index+1) }
    }

    // /// If this expression is a Tuple, get its expressions. Otherwise throw an error 
    // pub fn as_vec<'a>(&'a self) -> Result<&'a C,()> { 
    //     match self {
    //         TblExpression::Atomic(_) => Err(()),
    //         TblExpression::Compound(proposition_exprs) => Ok(proposition_exprs),
    //     }
    // }

    /// If this expression is a Tuple, get its subexpressions. Otherwise throw an error 
    pub fn as_slice(&self) -> Result<&[UnassignedTblExpression<C>], ()> {
        match self {
            UnassignedTblExpression::Compound(proposition_exprs) => Ok(proposition_exprs.as_slice()),
            _ => Err(()),
        }
    }

    pub fn len(&self) -> Option<usize> {
        match self {
            UnassignedTblExpression::Compound(exprs) => Some(exprs.len()),
            _ => None,
        }
    }
}

impl <C: UnassignedCompoundTblExpression> TryInto<AtomicTblExpression> for UnassignedTblExpression<C> {
    type Error = ();
    fn try_into(self) -> Result<AtomicTblExpression, Self::Error> { match self {
        UnassignedTblExpression::Atomic(atom) => Ok(atom),
        _ => Err(()),
    }}
}
// impl <C: CompoundTblExpression> TryInto<C> for TblExpression<C> {
//     type Error = ();
//     fn try_into(self) -> Result<C, Self::Error> { match self {
//         TblExpression::Atomic(_) => Err(()),
//         TblExpression::Compound(compound) => Ok(compound),
//     }}
// }

impl <C:UnassignedCompoundTblExpression> ParentOfImmediateUnassignedSubexpressions<C> for UnassignedTblExpression<C> {
    fn get_immediate_subexpression_paths(&self) -> impl IntoIterator<Item = ImmediateSubexpressionInExpressionPath> { match self {
        UnassignedTblExpression::Compound(compound) => compound.get_immediate_subexpression_paths().into_iter().collect(),
        _ => Box::from_iter([]),
    }}

    fn get_immediate_subexpression(&self,path: &ImmediateSubexpressionInExpressionPath) -> Result<&UnassignedTblExpression<C> ,()>  { match self {
        UnassignedTblExpression::Compound(c) => c.get_immediate_subexpression(path),
        _ => Err(()),
    }}
}
impl <C:UnassignedCompoundTblExpression> ParentOfUnassignedSubexpressions<C> for UnassignedTblExpression<C> {
    fn get_subexpression_paths(&self) -> impl IntoIterator<Item = TblSubexpressionInExpressionPath> {
        let immediate = self.get_immediate_subexpression_paths()
            .into_iter()
            .map(|x| x.into());
        let deferred = self.get_located_immediate_subexpressions()
            .into_iter()
            .map(|inner| inner.obj.get_subexpression_paths()
                .into_iter()
                .map(|p| (inner.path,p).into())
                .collect::<Vec<_>>()
            ).flatten();
        immediate.chain(deferred)
    }

    fn get_subexpression(&self,path: &TblSubexpressionInExpressionPath) -> Result< &UnassignedTblExpression<C> ,()>
        { self.get_subexpressions_helper(path, 0) }
}

mod from {
    use std::{rc::Rc, sync::Arc};

    use crate::{expressions::unassigned::{UnassignedTblExpression, compound::UnassignedCompoundTblExpression, variable::TblExpressionVariable}, expressions::assigned::{TblExpression, atomic::AtomicTblExpression, compound::CompoundTblExpression}};

    impl <C: CompoundTblExpression, UC: UnassignedCompoundTblExpression> From<TblExpression<C>> for UnassignedTblExpression<UC> where UC: From<C> {
        fn from(value: TblExpression<C>) -> Self { match value {
            TblExpression::Atomic(atom) => UnassignedTblExpression::Atomic(atom),
            TblExpression::Compound(compound) => UnassignedTblExpression::Compound(compound.into()),
        }}
    }
    impl <C: CompoundTblExpression, UC: UnassignedCompoundTblExpression> TryInto<TblExpression<C>> for UnassignedTblExpression<UC> where UC: TryInto<C,Error=usize> {
        type Error = TblExpressionVariable;
    
        fn try_into(self) -> Result<TblExpression<C>, Self::Error> { match self {
            UnassignedTblExpression::Atomic(atom) => Ok(TblExpression::Atomic(atom)),
            UnassignedTblExpression::Compound(compound) => Ok(TblExpression::Compound(compound.try_into()?)),
            UnassignedTblExpression::Variable(variable) => Err(variable),
        }}
    }


    impl <C: UnassignedCompoundTblExpression> From<AtomicTblExpression> for UnassignedTblExpression<C> {
        fn from(id: AtomicTblExpression) -> Self
            { Self::Atomic(id) }
    }
    impl <C: UnassignedCompoundTblExpression> From<u16> for UnassignedTblExpression<C> {
        fn from(id: u16) -> Self
            { AtomicTblExpression(id).into() }
    }
    impl <C: UnassignedCompoundTblExpression> From<C> for UnassignedTblExpression<C> {
        fn from(expr: C) -> Self
            { Self::Compound(expr) }
    }
    impl <const N: usize, C: UnassignedCompoundTblExpression> From<[TblExpression<C>;N]> for UnassignedTblExpression<C> where C: From<[UnassignedTblExpression<C>;N]> {
        fn from(exprs: [UnassignedTblExpression<C>;N]) -> Self
            { C::from(exprs).into() }
    }
    impl <C: UnassignedCompoundTblExpression> From<Box<[UnassignedTblExpression<C>]>> for UnassignedTblExpression<C> where C: From<Box<[UnassignedTblExpression<C>]>> {
        fn from(exprs: Box<[UnassignedTblExpression<C>]>) -> Self
            { C::from(exprs).into() }
    }
    impl <C: UnassignedCompoundTblExpression> From<Rc<[TblExpression<C>]>> for UnassignedTblExpression<C> where C: From<Rc<[UnassignedTblExpression<C>]>> {
        fn from(exprs: Rc<[UnassignedTblExpression<C>]>) -> Self
            { C::from(exprs).into() }
    }
    impl <C: UnassignedCompoundTblExpression> From<Arc<[TblExpression<C>]>> for UnassignedTblExpression<C> where C: From<Arc<[UnassignedTblExpression<C>]>> {
        fn from(exprs: Arc<[UnassignedTblExpression<C>]>) -> Self
            { C::from(exprs).into() }
    }
    impl <C: UnassignedCompoundTblExpression> From<Vec<TblExpression<C>>> for UnassignedTblExpression<C> where C: From<Vec<UnassignedTblExpression<C>>> {
        fn from(exprs: Vec<UnassignedTblExpression<C>>) -> Self
            { C::from(exprs).into() }
    }
}
