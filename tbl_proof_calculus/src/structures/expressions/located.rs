use path_lib::obj_at_path::{ObjAtPath, OwnedObjAtPath};

use crate::structures::expressions::{TblExpression, compound::CompoundTblExpression};

pub type TblExpressionAtPath<'a,C: CompoundTblExpression, Path> = ObjAtPath<'a,TblExpression<C>,Path>;
pub type OwnedTblExpressionAtPath<C: CompoundTblExpression, Path> = OwnedObjAtPath<TblExpression<C>,Path>;
