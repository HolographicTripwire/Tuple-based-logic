use path_lib::obj_at_path::ObjAtPath;

use crate::expressions::paths::{TblSubexpressionInExpressionPath, immediate::ImmediateTblSubexpressionInExpressionPath};

pub struct ExpressionObjLocator<T>(ExpressionObjLocatorInner<T>);

impl <T> ExpressionObjLocator<T> {
    pub fn new() -> Self { Self(ExpressionObjLocatorInner::Search(Vec::new())) }
    pub fn get(&self, path: TblSubexpressionInExpressionPath) -> Option<&T>
        { self.0.get(path.0.iter()) }
    pub fn get_with_path(&self, path: TblSubexpressionInExpressionPath) -> Option<ObjAtPath<T,TblSubexpressionInExpressionPath>>
        { self.0.get_with_path(path.0.iter()) }
    pub fn insert(&mut self, obj: T, path: TblSubexpressionInExpressionPath)
        { self.0.insert(obj, path.0.iter()); }
}

pub enum ExpressionObjLocatorInner<T> {
    Obj(T),
    Search(Vec<Self>)
}

impl <T> ExpressionObjLocatorInner<T> {
    pub fn get<'a, I: Iterator<Item=&'a ImmediateTblSubexpressionInExpressionPath>>(&self, mut paths: I) -> Option<&T> {
        match self {
            ExpressionObjLocatorInner::Obj(obj) => Some(obj),
            ExpressionObjLocatorInner::Search(subobjects) =>
                { subobjects.get(paths.next()?.0)?.get(paths) },
        }
    }
    pub fn get_with_path<'a, I: Iterator<Item=&'a ImmediateTblSubexpressionInExpressionPath>>(&self, mut paths: I) -> Option<ObjAtPath<T,TblSubexpressionInExpressionPath>> {
        match self {
            ExpressionObjLocatorInner::Obj(obj) => Some(ObjAtPath { obj, path: paths.copied().collect::<Vec<_>>().into() }),
            ExpressionObjLocatorInner::Search(subobjects) =>
                { subobjects.get(paths.next()?.0)?.get_with_path(paths) }
        }
    }
    /// Returns:
    /// Some((existing,attempted)) if an object already existed as a parent - where existing is the existing object and attempting is the object that you tried to insert
    /// None if the object was successfully inserted
    fn insert<'a, I: Iterator<Item=&'a ImmediateTblSubexpressionInExpressionPath>>(&mut self, obj: T, mut paths: I) -> Option<(&mut T,T)> {
        match self {
            ExpressionObjLocatorInner::Obj(existing) => Some((existing, obj)),
            ExpressionObjLocatorInner::Search(subobjects) => {
                let subpath = paths.next()?;
                if subpath.0 >= subobjects.len()
                    { subobjects.resize_with(subpath.0, || ExpressionObjLocatorInner::Search(Vec::new())); }
                let inner = &mut subobjects[subpath.0];
                inner.insert(obj, paths)
            }
        }
    }
}
