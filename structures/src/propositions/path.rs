
use path_lib::{paths::{PathPrimitive, PathSeries}, HasChildren};

use crate::propositions::Expression;

#[derive(Clone)]
pub struct AtomicSubexpressionPath(usize);
impl PathPrimitive for AtomicSubexpressionPath {}
impl From<usize> for AtomicSubexpressionPath {
    fn from(value: usize) -> Self { Self(value) }
}
pub type SubexpressionPath = PathSeries<AtomicSubexpressionPath>;

impl <'a> HasChildren<'a,AtomicSubexpressionPath,Expression> for Expression {
    fn children(&'a self) -> impl IntoIterator<Item = &'a Expression> {
        if let Ok(vec) = self.as_vec()
            { vec.iter().collect() }
        else { vec![] }
    }

    fn get_child(&'a self, path: &AtomicSubexpressionPath) -> Result<&'a Expression,()> {
        self.as_vec()?.get(path.0).ok_or(())
    }
}

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
}
