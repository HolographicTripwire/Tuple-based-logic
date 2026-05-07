use path_lib::obj_at_path::{ObjAtPath, OwnedObjAtPath};
use proof_calculus::propositions::types::unassigned::UnassignedProposition;

use crate::expressions::{paths::{TblSubexpressionInExpressionPath, immediate::ImmediateTblSubexpressionInExpressionPath}, types::{assigned::atom::TblExpressionAtom, unassigned::{compound::{UnassignedTblExpressionCompound, arc::UnassignedArcCompoundTblExpression, r#box::UnassignedBoxCompoundTblExpression, rc::UnassignedRcCompoundTblExpression}, subexpressions::{ParentOfUnassignedSubexpressions, UnassignedTblSubexpressionInExpression, immediate::ParentOfImmediateUnassignedSubexpressions, iterators::depth_first::counterclockwise::{CounterclockwiseDepthFirstLocatedUnassignedTblSubexpressionIterator, CounterclockwiseDepthFirstUnassignedTblSubexpressionIterator}}, variable::TblExpressionVariable}}};

pub mod variable;
pub mod compound;
pub mod subexpressions;
pub mod at_path_enum;
pub mod binding;

#[derive(Clone,PartialEq,Eq,Hash,Debug)]
pub enum UnassignedTblExpression<C: UnassignedTblExpressionCompound> {
    Atom(TblExpressionAtom),
    Variable(TblExpressionVariable),
    Compound(C),
}
pub type BoxUnassignedTblExpression = UnassignedTblExpression<UnassignedBoxCompoundTblExpression>;
pub type RcUnassignedTblExpression = UnassignedTblExpression<UnassignedRcCompoundTblExpression>;
pub type ArcUnassignedTblExpression = UnassignedTblExpression<UnassignedArcCompoundTblExpression>;

pub type UnassignedTblExpressionAtPath<'a,C: UnassignedTblExpressionCompound, Path> = ObjAtPath<'a,UnassignedTblExpression<C>,Path>;
pub type BoxUnassignedTblExpressionAtPath<'a,Path> = UnassignedTblExpressionAtPath<'a,UnassignedBoxCompoundTblExpression,Path>;
pub type RcUnassignedTblExpressionAtPath<'a,Path> = UnassignedTblExpressionAtPath<'a,UnassignedRcCompoundTblExpression,Path>;
pub type ArcUnassignedTblExpressionAtPath<'a,Path> = UnassignedTblExpressionAtPath<'a,UnassignedArcCompoundTblExpression,Path>;

pub type OwnedUnassignedTblExpressionAtPath<C: UnassignedTblExpressionCompound, Path> = OwnedObjAtPath<UnassignedTblExpression<C>,Path>;
pub type OwnedBoxUnassignedTblExpressionAtPath<Path> = OwnedUnassignedTblExpressionAtPath<UnassignedBoxCompoundTblExpression,Path>;
pub type OwnedRcUnassignedTblExpressionAtPath<Path> = OwnedUnassignedTblExpressionAtPath<UnassignedRcCompoundTblExpression,Path>;
pub type OwnedArcUnassignedTblExpressionAtPath<Path> = OwnedUnassignedTblExpressionAtPath<UnassignedArcCompoundTblExpression,Path>;

impl <C: UnassignedTblExpressionCompound> UnassignedTblExpression<C> {
    pub fn replace(&self, to_replace: &UnassignedTblExpression<C>, replace_with: &UnassignedTblExpression<C>) -> Self {
        if self == to_replace { replace_with.clone() }
        else if let UnassignedTblExpression::Compound(compound) = self
            { UnassignedTblExpression::Compound(compound.replace(to_replace, replace_with)) }
        else { self.clone() }
    }

    pub fn is_atom(&self) -> bool { if let UnassignedTblExpression::Atom(_) = self { true } else { false } }
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
    pub fn as_slice(&self) -> Result<&[UnassignedTblExpression<C>], ()> { match self {
        UnassignedTblExpression::Compound(proposition_exprs) => Ok(proposition_exprs.as_slice()),
        _ => Err(()),
    }}

    pub fn len(&self) -> Option<usize> { match self {
        UnassignedTblExpression::Compound(exprs) => Some(exprs.len()),
        _ => None,
    }}
}
impl <C: UnassignedTblExpressionCompound> UnassignedProposition for UnassignedTblProposition<C> {
    type DefaultPartialAssignment<'slf>;
    type DefaultNormalisation;

    fn partial_assign<'slf, PartialAssignment: proof_calculus::propositions::assignments::PartialPropositionalAssignment<'slf,'slf,Self,Self>>(self, assignment: &PartialAssignment) -> Self {
        todo!()
    }

    fn partial_reverse_assign<'slf>(&self, assigned: &Self) -> Result<Self::DefaultPartialAssignment<'slf>,()> {
        todo!()
    }

    fn normalise(self) -> Self::DefaultNormalisation {
        todo!()
    }
}

impl <C: UnassignedTblExpressionCompound> TryInto<TblExpressionAtom> for UnassignedTblExpression<C> {
    type Error = ();
    fn try_into(self) -> Result<TblExpressionAtom, Self::Error> { match self {
        UnassignedTblExpression::Atom(atom) => Ok(atom),
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

impl <C:UnassignedTblExpressionCompound> ParentOfImmediateUnassignedSubexpressions<C> for UnassignedTblExpression<C> {
    fn get_immediate_subexpression_paths(&self) -> impl IntoIterator<Item = ImmediateTblSubexpressionInExpressionPath> { match self {
        UnassignedTblExpression::Compound(compound) => compound.get_immediate_subexpression_paths().into_iter().collect(),
        _ => Box::from_iter([]),
    }}
    fn get_immediate_subexpression(&self,path: &ImmediateTblSubexpressionInExpressionPath) -> Result<&UnassignedTblExpression<C> ,()>  { match self {
        UnassignedTblExpression::Compound(c) => c.get_immediate_subexpression(path),
        _ => Err(()),
    }}
}
impl <C:UnassignedTblExpressionCompound> ParentOfUnassignedSubexpressions<C> for UnassignedTblExpression<C> {
    fn get_subexpression_paths(&self) -> impl IntoIterator<Item = TblSubexpressionInExpressionPath>
        { self.get_located_subexpressions().into_iter().map(|expr| expr.path) }
    fn get_subexpression(&self,path: &TblSubexpressionInExpressionPath) -> Result< &UnassignedTblExpression<C> ,()>
        { self.get_subexpressions_helper(path, 0) }
    
    fn get_subexpressions<'a>(&'a self) -> impl IntoIterator<Item =  &'a UnassignedTblExpression<C> > where UnassignedTblExpression<C> :'a
        { CounterclockwiseDepthFirstUnassignedTblSubexpressionIterator::new(self) }
    fn get_located_subexpressions<'a>(&'a self) -> impl IntoIterator<Item = UnassignedTblSubexpressionInExpression<'a,C>> where UnassignedTblExpression<C> :'a
        { CounterclockwiseDepthFirstLocatedUnassignedTblSubexpressionIterator::new(self) }
}

mod from {
    use std::{rc::Rc, sync::Arc};

    use crate::expressions::types::{assigned::{TblExpression, atom::TblExpressionAtom, compound::TblExpressionCompound}, unassigned::{UnassignedTblExpression, compound::UnassignedTblExpressionCompound, variable::TblExpressionVariable}};

    impl <C: TblExpressionCompound, UC: for<'a> From<&'a C> + UnassignedTblExpressionCompound> From<&TblExpression<C>> for UnassignedTblExpression<UC> {
        fn from(value: &TblExpression<C>) -> Self { match value {
            TblExpression::Atom(atom) => UnassignedTblExpression::Atom(*atom),
            TblExpression::Compound(compound) => UnassignedTblExpression::Compound(compound.into()),
        }}
    }
    impl <C: TblExpressionCompound, UC: UnassignedTblExpressionCompound> TryInto<TblExpression<C>> for &UnassignedTblExpression<UC> where for<'a> &'a UC: TryInto<C,Error=TblExpressionVariable> {
        type Error = TblExpressionVariable;
        fn try_into(self) -> Result<TblExpression<C>, Self::Error> { match self {
            UnassignedTblExpression::Atom(atom) => Ok(TblExpression::Atom(*atom)),
            UnassignedTblExpression::Compound(compound) => Ok(TblExpression::Compound(compound.try_into()?)),
            UnassignedTblExpression::Variable(variable) => Err(*variable),
        }}
    }
    impl <C1: UnassignedTblExpressionCompound, C2: UnassignedTblExpressionCompound + for<'a> From<&'a C1>> From<&UnassignedTblExpression<C1>>
    for UnassignedTblExpression<C2> { fn from(value: &UnassignedTblExpression<C1>) -> Self { match value {
        UnassignedTblExpression::Atom(atom) => UnassignedTblExpression::Atom(*atom),
        UnassignedTblExpression::Variable(variable) => UnassignedTblExpression::Variable(*variable),
        UnassignedTblExpression::Compound(compound) => UnassignedTblExpression::Compound(compound.into()),
    }}}

    impl <UC: UnassignedTblExpressionCompound> From<TblExpressionAtom> for UnassignedTblExpression<UC> 
        { fn from(id: TblExpressionAtom) -> Self { Self::Atom(id) } }
    impl <UC: UnassignedTblExpressionCompound> From<u16> for UnassignedTblExpression<UC> 
        { fn from(id: u16) -> Self { TblExpressionAtom(id).into() } }
    impl <UC: UnassignedTblExpressionCompound> From<UC> for UnassignedTblExpression<UC> 
        { fn from(expr: UC) -> Self { Self::Compound(expr) } }
    impl <const N: usize, C: UnassignedTblExpressionCompound> From<[Self;N]> for UnassignedTblExpression<C> where C: From<[Self;N]>
        { fn from(exprs: [Self;N]) -> Self { C::from(exprs).into() } }
    impl <UC: UnassignedTblExpressionCompound> From<Box<[Self]>> for UnassignedTblExpression<UC> where UC: From<Box<[Self]>> 
        { fn from(exprs: Box<[Self]>) -> Self { UC::from(exprs).into() } }
    impl <UC: UnassignedTblExpressionCompound> From<Rc<[Self]>> for UnassignedTblExpression<UC> where UC: From<Rc<[Self]>> 
        { fn from(exprs: Rc<[Self]>) -> Self { UC::from(exprs).into() } }
    impl <UC: UnassignedTblExpressionCompound> From<Arc<[Self]>> for UnassignedTblExpression<UC> where UC: From<Arc<[Self]>> 
        { fn from(exprs: Arc<[Self]>) -> Self { UC::from(exprs).into() } }
    impl <UC: UnassignedTblExpressionCompound> From<Vec<Self>> for UnassignedTblExpression<UC> where UC: From<Vec<Self>> 
        { fn from(exprs: Vec<Self>) -> Self { UC::from(exprs).into() } }
    
    impl <UC: UnassignedTblExpressionCompound + FromIterator<Self>> FromIterator<Self> for UnassignedTblExpression<UC>
        { fn from_iter<T: IntoIterator<Item = Self>>(iter: T) -> Self { Self::Compound(UC::from_iter(iter)) } }
}

#[derive(Clone,Copy,PartialEq,Eq,Debug,Hash)]
pub enum AtomOrVariableOrCompoundLength {
    Atom(TblExpressionAtom),
    Variable(TblExpressionVariable),
    CompoundLength(usize)
}
