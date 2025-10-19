
use std::fmt::Display;

use path_lib::{obj_at_path::{ObjAtPath, OwnedObjAtPath}, paths::{PathPrimitive, PathSeries}, HasChildren};

use crate::{expressions::Expression, DisplayExt};

/// The atomic object that makes up [SubexpressionPaths](SubexpressionPath)
/// For example, within the [Expression] (a,(b,c),d), the [AtomicSubexpressionPath] 1 would lead to the [Expression] (b,c)
#[derive(Clone,Copy,PartialEq,Eq,Hash,Debug)]
pub struct AtomicExpressionInExpressionPath(usize);
impl PathPrimitive for AtomicExpressionInExpressionPath {}
/// A path to one [Expression], within another [Expression]
/// For example, within the [Expression] (a,(b,c),d), the [AtomicSubexpressionPath] (1,0) would lead to the [Expression] (b)
pub type ExpressionInExpressionPath = PathSeries<AtomicExpressionInExpressionPath>;

impl <'a> HasChildren<'a,AtomicExpressionInExpressionPath,Expression> for Expression {
    fn valid_primitive_paths(&self) -> impl IntoIterator<Item = AtomicExpressionInExpressionPath> {
        let max = if let Ok(vec) = self.as_vec()
            { vec.len() } else { 0 };
        (0..max).map(|ix| ix.into())
    }

    fn get_child(&'a self, path: &AtomicExpressionInExpressionPath) -> Result<&'a Expression,()>
        { self.as_vec()?.get(path.0).ok_or(()) }    
    fn get_child_owned(&self, path: &AtomicExpressionInExpressionPath) -> Result<Expression,()> where Expression: Clone
        { self.as_vec()?.get(path.0).ok_or(()).cloned() }
}

impl Display for AtomicExpressionInExpressionPath {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f,"{}",self.0)
    }
}

impl DisplayExt for ExpressionInExpressionPath {
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
    
    impl From<usize> for AtomicExpressionInExpressionPath {
        fn from(value: usize) -> Self { Self(value) }
    }
}

/// A reference to an [Expression], located within another [Expression] by a [SubexpressionPath]
pub type ExpressionInExpression<'a> = ObjAtPath<'a,Expression,ExpressionInExpressionPath>;
/// An [Expression], located within another [Expression] by a [SubexpressionPath]
pub type OwnedExpressionInExpression = OwnedObjAtPath<Expression,ExpressionInExpressionPath>;

#[cfg(test)]
mod tests {
    use path_lib::HasDescendants;

    use crate::atoms::AtomId;

    use super::*;

    #[test]
    fn test_get_subexpr_on_atom() {
        for i in 0..10 {
            let atomic_expr = Expression::from(AtomId(i));
            let path: ExpressionInExpressionPath = [0].into();
            assert_eq!(atomic_expr.get_descendant(&path), Err(()));
        }
    }

    #[test]
    fn test_get_subexpr_on_tuple() {
        for i in 0..10 {
            let atomic_expr = Expression::from(vec![Expression::from(AtomId(i))]);
            let path: ExpressionInExpressionPath = [0].into();
            assert_eq!(atomic_expr.get_descendant(&path), Ok(&Expression::from(AtomId(i))));
        }
    }

    #[test]
    fn test_get_subexpr_on_short_tuple() {
        for i in 0..10 {
            let atomic_expr = Expression::from(vec![Expression::from(AtomId(i))]);
            let path: ExpressionInExpressionPath = [1].into();
            assert_eq!(atomic_expr.get_descendant(&path), Err(()));
        }
    }

    #[test]
    fn test_display_on_atomic_path() {
        for i in 0..10 {
            let path: AtomicExpressionInExpressionPath = i.into();
            assert_eq!(path.to_string(), format!("{}",i));
        }
    }

    #[test]
    fn test_display_on_unary_path() {
        for i in 0..10 {
            let path: ExpressionInExpressionPath = [i].into();
            assert_eq!(path.display(), format!("{}",i));
        }
    }

    #[test]
    fn test_display_on_binary_path() {
        for i in 0..10 {
            for j in 0..10 {
                let path: ExpressionInExpressionPath = [i,j].into();
                assert_eq!(path.display(), format!("{}.{}",i,j));
            }
        }
    }

    #[test]
    fn test_display_on_ternary_path() {
        for i in 0..10 {
            for j in 0..10 {
                for k in 0..10 {
                    let path: ExpressionInExpressionPath = [i,j,k].into();
                    assert_eq!(path.display(), format!("{}.{}.{}",i,j,k));
                }
            }
        }
    }
}
