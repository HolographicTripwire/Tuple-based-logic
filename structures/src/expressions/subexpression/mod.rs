use std::fmt::Display;

use path_lib_proc_macros::generate_parent_of_children_trait;

use crate::expressions::{Expression, subexpression::immediate::ImmediateExpressionInExpressionPath};

pub mod immediate;

/// A path to one [Expression], within another [Expression]
/// For example, within the [Expression] (a,(b,c),d), the [AtomicSubexpressionPath] (1,0) would lead to the [Expression] (b)
#[derive(Clone,Debug,Default,PartialEq,Eq,Hash)]
pub struct ExpressionInExpressionPath(pub Vec<ImmediateExpressionInExpressionPath>);
pub type ExpressionInPropositionPath = ExpressionInExpressionPath;

generate_parent_of_children_trait!{
    (Expression), ExpressionInExpressionPath,
    "subexpression", "subexpressions", "Subexpressions"
}


impl Display for ImmediateExpressionInExpressionPath {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f,"{}",self.0)
    }
}

impl Display for ExpressionInExpressionPath {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f,"{}",self.0.iter()
            .map(|a| a.to_string())
            .collect::<Vec<String>>()
            .join(".")
        )
    }
}

mod from {
    use super::*;
    
    impl From<usize> for ExpressionInExpressionPath {
        fn from(value: usize) -> Self { value.into() }
    }
    impl From<ImmediateExpressionInExpressionPath> for ExpressionInExpressionPath {
        fn from(value: ImmediateExpressionInExpressionPath) -> Self { vec![value].into() }
    }
    impl From<(ImmediateExpressionInExpressionPath,ImmediateExpressionInExpressionPath)> for ExpressionInExpressionPath {
        fn from(value: (ImmediateExpressionInExpressionPath,ImmediateExpressionInExpressionPath)) -> Self { vec![value.0,value.1].into() }
    }
    impl From<Vec<ImmediateExpressionInExpressionPath>> for ExpressionInExpressionPath {
        fn from(value: Vec<ImmediateExpressionInExpressionPath>) -> Self { Self(value) }
    }

    impl From<(ExpressionInExpressionPath,ImmediateExpressionInExpressionPath)> for ExpressionInExpressionPath {
        fn from(mut value: (ExpressionInExpressionPath,ImmediateExpressionInExpressionPath)) -> Self {
            value.0.0.push(value.1);
            value.0
        }
    }
    impl From<(ImmediateExpressionInExpressionPath,ExpressionInExpressionPath)> for ExpressionInExpressionPath {
        fn from(mut value: (ImmediateExpressionInExpressionPath,ExpressionInExpressionPath)) -> Self {
            value.1.0.push(value.0);
            value.1
        }
    }
    impl From<(ExpressionInExpressionPath,ExpressionInExpressionPath)> for ExpressionInExpressionPath {
        fn from(mut value: (ExpressionInExpressionPath,ExpressionInExpressionPath)) -> Self {
            value.0.0.append(&mut value.1.0);
            value.0
        }
    }
}


#[cfg(test)]
mod tests {
    use crate::expressions::{CompoundExpression, atomic::AtomicExpression};

    use super::*;

    // #[test]
    // fn test_get_subexpr_on_atom() {
    //     for i in 0..10 {
    //         let atomic_expr = Expression::from(AtomicExpression(i));
    //         let path: ExpressionInExpressionPath = 0.into();
    //         assert_eq!(atomic_expr, Err(()));
    //     }
    // }

    #[test]
    fn test_get_subexpr_on_tuple() {
        for i in 0..10 {
            let atomic_expr = CompoundExpression::from(vec![Expression::from(AtomicExpression(i))]);
            let path: ExpressionInExpressionPath = 0.into();
            assert_eq!(atomic_expr.get_subexpression(&path), Ok(&Expression::from(AtomicExpression(i))));
        }
    }

    // #[test]
    // fn test_get_subexpr_on_short_tuple() {
    //     for i in 0..10 {
    //         let atomic_expr = Expression::from(vec![Expression::from(AtomicExpression(i))]);
    //         let path: ExpressionInExpressionPath = [1].into();
    //         assert_eq!(atomic_expr.get_descendant(&path), Err(()));
    //     }
    // }

    #[test]
    fn test_display_on_atomic_path() {
        for i in 0..10 {
            let path: ImmediateExpressionInExpressionPath = i.into();
            assert_eq!(path.to_string(), format!("{}",i));
        }
    }

    #[test]
    fn test_display_on_unary_path() {
        for i in 0..10 {
            let path: ExpressionInExpressionPath = vec![i.into()].into();
            assert_eq!(path.to_string(), format!("{}",i));
        }
    }

    #[test]
    fn test_display_on_binary_path() {
        for i in 0..10 {
            for j in 0..10 {
                let path: ExpressionInExpressionPath = vec![i.into(),j.into()].into();
                assert_eq!(path.to_string(), format!("{}.{}",i,j));
            }
        }
    }

    #[test]
    fn test_display_on_ternary_path() {
        for i in 0..10 {
            for j in 0..10 {
                for k in 0..10 {
                    let path: ExpressionInExpressionPath = vec![i.into(),j.into(),k.into()].into();
                    assert_eq!(path.to_string(), format!("{}.{}.{}",i,j,k));
                }
            }
        }
    }
}
