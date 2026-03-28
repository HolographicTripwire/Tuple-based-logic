use std::fmt::Display;

use path_lib::{obj_at_path::{ObjAtPath, OwnedObjAtPath}};
use path_lib_proc_macros::generate_parent_of_children_trait;

use crate::structures::expressions::{TblExpression, compound::CompoundTblExpression, subexpressions::immediate::ImmediateSubexpressionInExpressionPath};

pub mod immediate;

#[derive(Clone,PartialEq,Eq,Hash,Debug,Default)]
pub struct SubexpressionInExpressionPath(pub Vec<ImmediateSubexpressionInExpressionPath>);
impl Display for SubexpressionInExpressionPath {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0.iter()
            .map(|atom| atom.to_string())
            .collect::<Vec<String>>().join(".")
        )
    }
}

generate_parent_of_children_trait!{
    TblExpression<C>, SubexpressionInExpressionPath, (C: CompoundTblExpression),
    "subexpression", "subexpressions", "Subexpressions"
}

pub type SubexpressionInExpression<'a,C> = ObjAtPath<'a,TblExpression<C>,SubexpressionInExpressionPath>;
pub type OwnedSubexpressionInExpression<C> = OwnedObjAtPath<TblExpression<C>,SubexpressionInExpressionPath>;

mod from {
    use super::*;
    
    impl From<usize> for SubexpressionInExpressionPath {
        fn from(value: usize) -> Self { value.into() }
    }
    impl From<ImmediateSubexpressionInExpressionPath> for SubexpressionInExpressionPath {
        fn from(value: ImmediateSubexpressionInExpressionPath) -> Self { vec![value].into() }
    }
    impl From<(ImmediateSubexpressionInExpressionPath,ImmediateSubexpressionInExpressionPath)> for SubexpressionInExpressionPath {
        fn from(value: (ImmediateSubexpressionInExpressionPath,ImmediateSubexpressionInExpressionPath)) -> Self { vec![value.0,value.1].into() }
    }
    impl From<Vec<ImmediateSubexpressionInExpressionPath>> for SubexpressionInExpressionPath {
        fn from(value: Vec<ImmediateSubexpressionInExpressionPath>) -> Self { Self(value) }
    }

    impl From<(SubexpressionInExpressionPath,ImmediateSubexpressionInExpressionPath)> for SubexpressionInExpressionPath {
        fn from(mut value: (SubexpressionInExpressionPath,ImmediateSubexpressionInExpressionPath)) -> Self {
            value.0.0.push(value.1);
            value.0
        }
    }
    impl From<(ImmediateSubexpressionInExpressionPath,SubexpressionInExpressionPath)> for SubexpressionInExpressionPath {
        fn from(mut value: (ImmediateSubexpressionInExpressionPath,SubexpressionInExpressionPath)) -> Self {
            value.1.0.push(value.0);
            value.1
        }
    }
    impl From<(SubexpressionInExpressionPath,SubexpressionInExpressionPath)> for SubexpressionInExpressionPath {
        fn from(mut value: (SubexpressionInExpressionPath,SubexpressionInExpressionPath)) -> Self {
            value.0.0.append(&mut value.1.0);
            value.0
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::structures::expressions::atomic::AtomicTblExpression;

    use super::*;

    // #[test]
    // fn test_get_subexpr_on_atom() {
    //     for i in 0..10 {
    //         let atomic_expr = Expression::from(AtomicExpression(i));
    //         let path: ExpressionInExpressionPath = 0.into();
    //         assert_eq!(atomic_expr, Err(()));
    //     }
    // }

    // #[test]
    // fn test_get_subexpr_on_tuple() {
    //     for i in 0..10 {
    //         let atomic_expr = CompoundTblExpression::from(vec![TblExpression::from(AtomicTblExpression(i))]);
    //         let path: ExpressionInExpressionPath = 0.into();
    //         assert_eq!(atomic_expr.get_subexpression(&path), Ok(&TblExpression::from(AtomicTblExpression(i))));
    //     }
    // }

    // #[test]
    // fn test_get_subexpr_on_short_tuple() {
    //     for i in 0..10 {
    //         let atomic_expr = Expression::from(vec![Expression::from(AtomicExpression(i))]);
    //         let path: ExpressionInExpressionPath = [1].into();
    //         assert_eq!(atomic_expr.get_descendant(&path), Err(()));
    //     }
    // }

    // #[test]
    // fn test_display_on_atomic_path() {
    //     for i in 0..10 {
    //         let path: ImmediateExpressionInExpressionPath = i.into();
    //         assert_eq!(path.to_string(), format!("{}",i));
    //     }
    // }

    // #[test]
    // fn test_display_on_unary_path() {
    //     for i in 0..10 {
    //         let path: ExpressionInExpressionPath = vec![i.into()].into();
    //         assert_eq!(path.to_string(), format!("{}",i));
    //     }
    // }

    // #[test]
    // fn test_display_on_binary_path() {
    //     for i in 0..10 {
    //         for j in 0..10 {
    //             let path: ExpressionInExpressionPath = vec![i.into(),j.into()].into();
    //             assert_eq!(path.to_string(), format!("{}.{}",i,j));
    //         }
    //     }
    // }

    // #[test]
    // fn test_display_on_ternary_path() {
    //     for i in 0..10 {
    //         for j in 0..10 {
    //             for k in 0..10 {
    //                 let path: ExpressionInExpressionPath = vec![i.into(),j.into(),k.into()].into();
    //                 assert_eq!(path.to_string(), format!("{}.{}.{}",i,j,k));
    //             }
    //         }
    //     }
    // }
}
