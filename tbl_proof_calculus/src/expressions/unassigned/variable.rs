use path_lib::obj_at_path::{ObjAtPath, OwnedObjAtPath};

#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
pub struct TblExpressionVariable(pub usize);
impl TblExpressionVariable {
    pub fn first() -> Self { Self(0) }
    pub fn next(&self) -> Self { Self(self.0 + 1) }
}
impl From<usize> for TblExpressionVariable {
    fn from(value: usize) -> Self { Self(value) }
}
impl Into<usize> for TblExpressionVariable {
    fn into(self) -> usize { self.0 }
}

pub type TblExpressionVariableAtPath<'a,Path> = ObjAtPath<'a,TblExpressionVariable,Path>;
pub type OwnedTblExpressionVariableAtPath<Path> = OwnedObjAtPath<TblExpressionVariable,Path>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_id_initialisation() {
        let result = TblExpressionVariable::first();
        assert_eq!(result, TblExpressionVariable(0));
    }

    #[test]
    #[should_panic]
    fn test_id_overflow() {
        let result = TblExpressionVariable(usize::max_value());
        result.next();
    }
}
