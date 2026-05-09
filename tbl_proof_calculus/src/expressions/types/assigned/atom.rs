use std::num::TryFromIntError;

use path_lib::obj_at_path::{ObjAtPath, OwnedObjAtPath};

/// An [Identifier] used for Atom objects, which are used for building tuple objects in Tuple-based logic
#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
pub struct TblExpressionAtom(pub u16);
impl TblExpressionAtom {
    pub fn first() -> TblExpressionAtom {
        TblExpressionAtom(0)
    }
    pub fn next(&self) -> TblExpressionAtom {
        TblExpressionAtom(self.0 + 1)
    }
}
impl TryFrom<usize> for TblExpressionAtom {
    type Error = TryFromIntError;
    fn try_from(value: usize) -> Result<Self, Self::Error> {
        match u16::try_from(value) {
            Ok(val) => Ok(Self(val)),
            Err(err) => Err(err),
        }
    }
}
impl TryFrom<TblExpressionAtom> for usize {
    type Error = TryFromIntError;
    fn try_from(value: TblExpressionAtom) -> Result<Self, Self::Error> {
        Ok(usize::try_from(value.0)?)
    }
}

pub type TblExpressionAtomAtPath<'a, Path> = ObjAtPath<'a, TblExpressionAtom, Path>;
pub type OwnedTblExpressionAtomAtPath<Path> = OwnedObjAtPath<TblExpressionAtom, Path>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_id_initialisation() {
        let result = TblExpressionAtom::first();
        assert_eq!(result, TblExpressionAtom(0));
    }

    #[test]
    #[should_panic]
    fn test_id_overflow() {
        let result = TblExpressionAtom(u16::max_value());
        result.next();
    }
}
