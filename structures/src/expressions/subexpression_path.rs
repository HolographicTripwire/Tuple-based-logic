
use std::fmt::Display;

use path_lib::{obj_at_path::{ObjAtPath, OwnedObjAtPath}, paths::{PathPrimitive, PathSeries}, HasChildren};

use crate::{expressions::Expression, DisplayExt};

/// The atomic object that makes up [SubexpressionPaths](SubexpressionPath)
/// For example, within the [Expression] (a,(b,c),d), the [AtomicSubexpressionPath] 1 would lead to the [Expression] (b,c)
#[derive(Clone)]
pub struct AtomicSubexpressionPath(usize);
impl PathPrimitive for AtomicSubexpressionPath {}
/// A path to one [Expression], within another [Expression]
/// For example, within the [Expression] (a,(b,c),d), the [AtomicSubexpressionPath] (1,0) would lead to the [Expression] (b)
pub type SubexpressionPath = PathSeries<AtomicSubexpressionPath>;

impl <'a> HasChildren<'a,AtomicSubexpressionPath,Expression> for Expression {
    fn valid_primitive_paths(&self) -> impl IntoIterator<Item = AtomicSubexpressionPath> {
        let max = if let Ok(vec) = self.as_vec()
            { vec.len() }else { 0 };
        (0..max).map(|ix| ix.into())
    }

    fn get_child(&'a self, path: &AtomicSubexpressionPath) -> Result<&'a Expression,()>
        { self.as_vec()?.get(path.0).ok_or(()) }
}

impl Display for AtomicSubexpressionPath {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f,"{}",self.0)
    }
}

impl DisplayExt for SubexpressionPath {
    fn display(&self) -> String {
        self.paths()
            .iter()
            .map(|a| a.to_string())
            .collect::<Vec<String>>()
            .join(".")
    }
}

mod from {
    use super::*;
    
    impl From<usize> for AtomicSubexpressionPath {
        fn from(value: usize) -> Self { Self(value) }
    }
}

/// A reference to an [Expression], located within another [Expression] by a [SubexpressionPath]
pub type SubexpressionInExpression<'a> = ObjAtPath<'a,Expression,SubexpressionPath>;
/// An [Expression], located within another [Expression] by a [SubexpressionPath]
pub type OwnedSubexpressionInExpression = OwnedObjAtPath<Expression,SubexpressionPath>;

#[cfg(test)]
mod tests {
    use path_lib::HasDescendants;

    use crate::atoms::AtomId;

    use super::*;

    #[test]
    fn test_get_subexpr_on_atom() {
        for i in 0..10 {
            let atomic_expr = Expression::from(AtomId(i));
            let path: SubexpressionPath = [0].into();
            assert_eq!(atomic_expr.get_descendant(&path), Err(()));
        }
    }

    #[test]
    fn test_get_subexpr_on_tuple() {
        for i in 0..10 {
            let atomic_expr = Expression::from(vec![Expression::from(AtomId(i))]);
            let path: SubexpressionPath = [0].into();
            assert_eq!(atomic_expr.get_descendant(&path), Ok(&Expression::from(AtomId(i))));
        }
    }

    #[test]
    fn test_get_subexpr_on_short_tuple() {
        for i in 0..10 {
            let atomic_expr = Expression::from(vec![Expression::from(AtomId(i))]);
            let path: SubexpressionPath = [1].into();
            assert_eq!(atomic_expr.get_descendant(&path), Err(()));
        }
    }

    #[test]
    fn test_display_on_atomic_path() {
        for i in 0..10 {
            let path: AtomicSubexpressionPath = i.into();
            assert_eq!(path.to_string(), format!("{}",i));
        }
    }

    #[test]
    fn test_display_on_unary_path() {
        for i in 0..10 {
            let path: SubexpressionPath = [i].into();
            assert_eq!(path.display(), format!("{}",i));
        }
    }

    #[test]
    fn test_display_on_binary_path() {
        for i in 0..10 {
            for j in 0..10 {
                let path: SubexpressionPath = [i,j].into();
                assert_eq!(path.display(), format!("{}.{}",i,j));
            }
        }
    }

    #[test]
    fn test_display_on_ternary_path() {
        for i in 0..10 {
            for j in 0..10 {
                for k in 0..10 {
                    let path: SubexpressionPath = [i,j,k].into();
                    assert_eq!(path.display(), format!("{}.{}.{}",i,j,k));
                }
            }
        }
    }
}
