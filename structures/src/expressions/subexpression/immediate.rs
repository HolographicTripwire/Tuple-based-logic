use std::fmt::Display;

use path_lib::obj_at_path::{ObjAtPath, OwnedObjAtPath};
use path_lib_proc_macros::generate_parent_of_children_trait;

use crate::expressions::{Expression, at_path_enum::{ExpressionAtPathEnum, OwnedExpressionAtPathEnum}};

/// The atomic object that makes up [SubexpressionPaths](SubexpressionPath)
/// For example, within the [Expression] (a,(b,c),d), the [AtomicSubexpressionPath] 1 would lead to the [Expression] (b,c)
#[derive(Clone,Copy,PartialEq,Eq,Hash,Debug)]
pub struct ImmediateExpressionInExpressionPath(pub usize);
impl From<usize> for ImmediateExpressionInExpressionPath {
    fn from(value: usize) -> Self { Self(value) }
}
impl Display for ImmediateExpressionInExpressionPath {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
        { write!(f, "{}", self.0) }
}

// A reference to an [Expression], located within another [Expression] by a [SubexpressionPath]
// An [Expression], located within another [Expression] by a [SubexpressionPath]
trait Expr {}

pub trait ParentOfImmediateSubexpressions <>: Expr {
    fn get_immediate_subexpression_paths(& self) -> impl IntoIterator < Item =
    ImmediateExpressionInExpressionPath > ; fn
    get_immediate_subexpression(& self, path : &
    ImmediateExpressionInExpressionPath) -> Result < & Self, () > ; fn
    get_immediate_subexpressions < 'a > (& 'a self) -> impl IntoIterator <
    Item = & 'a Self >
    {
        self.get_immediate_subexpression_paths().into_iter().map(| path |
        self.get_immediate_subexpression(&
        path).expect("#get_child_paths returned an invalid path"))
    } fn
    get_immediate_subexpression_owned(& self, path : &
    ImmediateExpressionInExpressionPath) -> Result < Self, () > where Self :
    Clone { self.get_immediate_subexpression(path).cloned() } fn
    get_immediate_subexpressions_owned(& self) -> impl IntoIterator < Item =
    Self > where Self : Clone
    {
        self.get_immediate_subexpression_paths().into_iter().map(| path |
        self.get_immediate_subexpression_owned(&
        path).expect("#get_child_paths returned an invalid path"))
    } fn get_located_immediate_subexpression < 'a >
    (& 'a self, path : ImmediateExpressionInExpressionPath) -> Result <
    path_lib :: obj_at_path :: ObjAtPath < 'a, Self,
    ImmediateExpressionInExpressionPath > , () >
    {
        Ok(path_lib :: obj_at_path :: ObjAtPath
        { obj : self.get_immediate_subexpression(& path) ? , path })
    } fn get_located_immediate_subexpressions < 'a > (& 'a self) -> impl
    IntoIterator < Item = path_lib :: obj_at_path :: ObjAtPath < 'a, Self,
    ImmediateExpressionInExpressionPath >>
    {
        self.get_immediate_subexpression_paths().into_iter().map(| path |
        {
            self.get_located_immediate_subexpression(path).expect("#get_child_paths returned an invalid path")
        })
    } fn
    get_located_immediate_subexpression_owned(& self, path :
    ImmediateExpressionInExpressionPath) -> Result < path_lib :: obj_at_path
    :: OwnedObjAtPath < Self, ImmediateExpressionInExpressionPath > , () >
    where Self : Clone
    {
        Ok(path_lib :: obj_at_path :: OwnedObjAtPath
        { obj : self.get_immediate_subexpression_owned(& path) ? , path })
    } fn get_located_immediate_subexpressions_owned(& self) -> impl
    IntoIterator < Item = path_lib :: obj_at_path :: OwnedObjAtPath < Self,
    ImmediateExpressionInExpressionPath >> where Self : Clone
    {
        self.get_immediate_subexpression_paths().into_iter().map(| path |
        {
            self.get_located_immediate_subexpression_owned(path).expect("#get_child_paths returned an invalid path")
        })
    } fn into_located_immediate_subexpressions_owned(self) -> impl
    IntoIterator < Item = path_lib :: obj_at_path :: OwnedObjAtPath < Self,
    ImmediateExpressionInExpressionPath >> where Self : Clone, Self : Sized
    {
        self.get_located_immediate_subexpressions_owned().into_iter().collect
        :: < Vec < _ >> ()
    }
} pub trait LocatedParentOfImmediateSubexpressions < 'a, Parent, ParentPath,
JoinedPath : From < (ParentPath, ImmediateExpressionInExpressionPath) > , > where Parent : ParentOfImmediateSubexpressions <  > + 'a,
ParentPath : 'a + Clone
{
    fn self_ref(& self) -> & path_lib :: obj_at_path :: ObjAtPath < 'a,
    Parent, ParentPath > ; fn self_owned(self) -> path_lib :: obj_at_path ::
    ObjAtPath < 'a, Parent, ParentPath > ; fn
    get_immediate_subexpression_paths(& self) -> impl IntoIterator < Item =
    ImmediateExpressionInExpressionPath >
    { self.self_ref().obj.get_immediate_subexpression_paths() } fn
    get_joined_immediate_subexpression_paths(& self) -> impl IntoIterator <
    Item = JoinedPath >
    {
        self.get_immediate_subexpression_paths().into_iter().map(| path |
        self.self_ref().append_path(path))
    } fn
    get_immediate_subexpression(& 'a self, path : &
    ImmediateExpressionInExpressionPath) -> Result < & Parent, () >
    { self.self_ref().obj.get_immediate_subexpression(path) } fn
    get_immediate_subexpressions(& 'a self) -> impl IntoIterator < Item = & 'a
    Parent > { self.self_ref().obj.get_immediate_subexpressions() } fn
    get_immediate_subexpression_owned(& self, path : &
    ImmediateExpressionInExpressionPath) -> Result < Parent, () > where Parent :
    Clone { self.self_ref().obj.get_immediate_subexpression_owned(path) } fn
    get_immediate_subexpressions_owned(& self) -> impl IntoIterator < Item =
    Parent > where Parent : Clone
    { self.self_ref().obj.get_immediate_subexpressions_owned() } fn
    get_located_immediate_subexpression(& self, path :
    ImmediateExpressionInExpressionPath) -> Result < path_lib :: obj_at_path
    :: ObjAtPath < 'a, Parent, JoinedPath > , () >
    {
        let self_ref = self.self_ref();
        Ok(self_ref.obj.get_located_immediate_subexpression(path) ?
        .prepend_path_to_self(self_ref.path.clone()))
    } fn get_located_immediate_subexpressions(& 'a self) -> impl IntoIterator
    < Item = path_lib :: obj_at_path :: ObjAtPath < 'a, Parent, JoinedPath >>
    {
        let self_ref = self.self_ref();
        self_ref.obj.get_located_immediate_subexpressions().into_iter().map(|
        inner | inner.prepend_path_to_self(self_ref.path.clone()))
    } fn
    get_located_immediate_subexpression_owned(& self, path :
    ImmediateExpressionInExpressionPath) -> Result < path_lib :: obj_at_path
    :: OwnedObjAtPath < Parent, JoinedPath > , () > where Parent : Clone
    {
        let self_ref = self.self_ref();
        Ok(self_ref.obj.get_located_immediate_subexpression_owned(path) ?
        .prepend_path_to_self(self_ref.path.clone()))
    } fn get_located_immediate_subexpressions_owned(& 'a self) -> impl
    IntoIterator < Item = path_lib :: obj_at_path :: OwnedObjAtPath < Parent,
    JoinedPath >> where Parent : Clone
    {
        let self_ref = self.self_ref();
        self_ref.obj.get_located_immediate_subexpressions_owned().into_iter().map(|
        inner | inner.prepend_path_to_self(self_ref.path.clone()))
    }
} impl < 'a, Parent, ParentPath, JoinedPath, >
LocatedParentOfImmediateSubexpressions < 'a, Parent, ParentPath, JoinedPath, > for path_lib :: obj_at_path :: ObjAtPath < 'a, Parent, ParentPath >
where Parent : ParentOfImmediateSubexpressions <  > , ParentPath : 'a +
Clone, JoinedPath : From < (ParentPath, ImmediateExpressionInExpressionPath) >
{
    fn self_ref(& self) -> & path_lib :: obj_at_path :: ObjAtPath < 'a,
    Parent, ParentPath > { self } fn self_owned(self) -> path_lib ::
    obj_at_path :: ObjAtPath < 'a, Parent, ParentPath > { self }
} pub trait OwnedLocatedParentOfImmediateSubexpressions < 'a, Parent,
ParentPath, JoinedPath : From <
(ParentPath, ImmediateExpressionInExpressionPath) > , > where
Parent : 'a + Clone + ParentOfImmediateSubexpressions <  > , ParentPath :
'a + Clone
{
    fn self_ref(& self) -> & path_lib :: obj_at_path :: OwnedObjAtPath <
    Parent, ParentPath > ; fn self_owned(self) -> path_lib :: obj_at_path ::
    OwnedObjAtPath < Parent, ParentPath > ; fn
    get_immediate_subexpression_paths(& 'a self) -> impl IntoIterator < Item =
    ImmediateExpressionInExpressionPath > where Parent : 'a, ParentPath : 'a
    { self.self_ref().obj.get_immediate_subexpression_paths() } fn
    get_joined_immediate_subexpression_paths(& 'a self) -> impl IntoIterator <
    Item = JoinedPath >
    {
        self.get_immediate_subexpression_paths().into_iter().map(| path |
        self.self_ref().append_path(path))
    } fn
    get_immediate_subexpression(& 'a self, path : &
    ImmediateExpressionInExpressionPath) -> Result < & Parent, () >
    { self.self_ref().obj.get_immediate_subexpression(path) } fn
    get_immediate_subexpressions(& 'a self) -> impl IntoIterator < Item = & 'a
    Parent > { self.self_ref().obj.get_immediate_subexpressions() } fn
    get_immediate_subexpression_owned(& self, path : &
    ImmediateExpressionInExpressionPath) -> Result < Parent, () > where Parent :
    Clone { self.self_ref().obj.get_immediate_subexpression_owned(path) } fn
    get_immediate_subexpressions_owned(& 'a self) -> impl IntoIterator < Item
    = Parent > where Parent : Clone
    { self.self_ref().obj.get_immediate_subexpressions_owned() } fn
    get_located_immediate_subexpression(& 'a self, path :
    ImmediateExpressionInExpressionPath) -> Result < path_lib :: obj_at_path
    :: ObjAtPath < 'a, Parent, JoinedPath > , () >
    {
        let self_ref = self.self_ref();
        Ok(self_ref.obj.get_located_immediate_subexpression(path) ?
        .prepend_path_to_self(self_ref.path.clone()))
    } fn get_located_immediate_subexpressions(& 'a self) -> impl IntoIterator
    < Item = path_lib :: obj_at_path :: ObjAtPath < 'a, Parent, JoinedPath >>
    {
        let self_ref = self.self_ref();
        self_ref.obj.get_located_immediate_subexpressions().into_iter().map(|
        inner | inner.replace_path(| path | self_ref.append_path(path)))
    } fn
    get_located_immediate_subexpression_owned(& self, path :
    ImmediateExpressionInExpressionPath) -> Result < path_lib :: obj_at_path
    :: OwnedObjAtPath < Parent, JoinedPath > , () > where Parent : Clone
    {
        let self_ref = self.self_ref();
        Ok(self_ref.obj.get_located_immediate_subexpression_owned(path) ?
        .prepend_path_to_self(self_ref.path.clone()))
    } fn get_located_immediate_subexpressions_owned(& 'a self) -> impl
    IntoIterator < Item = path_lib :: obj_at_path :: OwnedObjAtPath < Parent,
    JoinedPath >> where Parent : Clone
    {
        let self_ref = self.self_ref();
        self_ref.obj.get_located_immediate_subexpressions_owned().into_iter().map(|
        inner | inner.prepend_path_to_self(self_ref.path.clone()))
    }
} impl < 'a, Parent, ParentPath, JoinedPath, >
OwnedLocatedParentOfImmediateSubexpressions < 'a, Parent, ParentPath,
JoinedPath,  > for path_lib :: obj_at_path :: OwnedObjAtPath < Parent,
ParentPath > where Parent : 'a + Clone + ParentOfImmediateSubexpressions <
 > , ParentPath : 'a + Clone, JoinedPath : From <
(ParentPath, ImmediateExpressionInExpressionPath) >
{
    fn self_ref(& self) -> & path_lib :: obj_at_path :: OwnedObjAtPath <
    Parent, ParentPath > { self } fn self_owned(self) -> path_lib ::
    obj_at_path :: OwnedObjAtPath < Parent, ParentPath > { self }
}

generate_parent_of_children_trait!{
    (Self where Self: Expr), ImmediateExpressionInExpressionPath,
    "immediate_subexpression", "immediate_subexpressions", "ImmediateSubexpressions"
}
pub type ImmediateSubexpressionInExpression<'a> = ObjAtPath<'a,Expression,ImmediateExpressionInExpressionPath>;
pub type ImmediateSubexpressionInExpressionEnum<'a> = ExpressionAtPathEnum<'a,ImmediateExpressionInExpressionPath>;

pub type OwnedImmediateSubexpressionInExpression = OwnedObjAtPath<Expression,ImmediateExpressionInExpressionPath>;
pub type OwnedImmediateSubexpressionInExpressionEnum = OwnedExpressionAtPathEnum<ImmediateExpressionInExpressionPath>;
