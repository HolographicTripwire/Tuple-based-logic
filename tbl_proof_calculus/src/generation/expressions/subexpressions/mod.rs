use path_lib::{obj_at_path::{ObjAtPath, OwnedObjAtPath}};
use path_lib_proc_macros::generate_parent_of_children_trait;

use crate::{generation::expressions::{UnassignedTblExpression, compound::UnassignedCompoundTblExpression}, structures::expressions::{subexpressions::TblSubexpressionInExpressionPath}};

pub mod immediate;

generate_parent_of_children_trait!{
    UnassignedTblExpression<C>, TblSubexpressionInExpressionPath, (C: UnassignedCompoundTblExpression),
    "subexpression", "subexpressions", "UnassignedSubexpressions"
}

pub type UnassignedTblSubexpressionInExpression<'a,C> = ObjAtPath<'a,UnassignedTblExpression<C>,TblSubexpressionInExpressionPath>;
pub type OwnedUnassignedTblSubexpressionInExpression<C> = OwnedObjAtPath<UnassignedTblExpression<C>,TblSubexpressionInExpressionPath>;

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
