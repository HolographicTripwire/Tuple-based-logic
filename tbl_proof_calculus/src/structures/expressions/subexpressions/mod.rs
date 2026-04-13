use std::fmt::Display;

use path_lib::{obj_at_path::{ObjAtPath, OwnedObjAtPath}};
use path_lib_proc_macros::generate_parent_of_children_trait;

use crate::structures::expressions::{TblExpression, compound::CompoundTblExpression, subexpressions::immediate::ImmediateSubexpressionInExpressionPath};

pub mod immediate;

#[derive(Clone,PartialEq,Eq,Hash,Debug,Default)]
pub struct TblSubexpressionInExpressionPath(pub Vec<ImmediateSubexpressionInExpressionPath>);
impl Display for TblSubexpressionInExpressionPath {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0.iter()
            .map(|atom| atom.to_string())
            .collect::<Vec<String>>().join(".")
        )
    }
}

generate_parent_of_children_trait!{
    TblExpression<C>, TblSubexpressionInExpressionPath, (C: CompoundTblExpression),
    "subexpression", "subexpressions", "Subexpressions"
}

pub type TblSubexpressionInExpression<'a,C> = ObjAtPath<'a,TblExpression<C>,TblSubexpressionInExpressionPath>;
pub type OwnedTblSubexpressionInExpression<C> = OwnedObjAtPath<TblExpression<C>,TblSubexpressionInExpressionPath>;

mod from {
    use super::*;
    
    impl From<usize> for TblSubexpressionInExpressionPath {
        fn from(value: usize) -> Self { value.into() }
    }
    impl From<ImmediateSubexpressionInExpressionPath> for TblSubexpressionInExpressionPath {
        fn from(value: ImmediateSubexpressionInExpressionPath) -> Self { vec![value].into() }
    }
    impl From<(ImmediateSubexpressionInExpressionPath,ImmediateSubexpressionInExpressionPath)> for TblSubexpressionInExpressionPath {
        fn from(value: (ImmediateSubexpressionInExpressionPath,ImmediateSubexpressionInExpressionPath)) -> Self { vec![value.0,value.1].into() }
    }
    impl From<Vec<ImmediateSubexpressionInExpressionPath>> for TblSubexpressionInExpressionPath {
        fn from(value: Vec<ImmediateSubexpressionInExpressionPath>) -> Self { Self(value) }
    }

    impl From<(TblSubexpressionInExpressionPath,ImmediateSubexpressionInExpressionPath)> for TblSubexpressionInExpressionPath {
        fn from(mut value: (TblSubexpressionInExpressionPath,ImmediateSubexpressionInExpressionPath)) -> Self {
            value.0.0.push(value.1);
            value.0
        }
    }
    impl From<(ImmediateSubexpressionInExpressionPath,TblSubexpressionInExpressionPath)> for TblSubexpressionInExpressionPath {
        fn from(mut value: (ImmediateSubexpressionInExpressionPath,TblSubexpressionInExpressionPath)) -> Self {
            value.1.0.insert(0,value.0);
            value.1
        }
    }
    impl From<(TblSubexpressionInExpressionPath,TblSubexpressionInExpressionPath)> for TblSubexpressionInExpressionPath {
        fn from(mut value: (TblSubexpressionInExpressionPath,TblSubexpressionInExpressionPath)) -> Self {
            value.0.0.append(&mut value.1.0);
            value.0
        }
    }
}

#[cfg(test)]
mod tests {
    // use crate::structures::expressions::atomic::AtomicTblExpression;

    // use super::*;

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
