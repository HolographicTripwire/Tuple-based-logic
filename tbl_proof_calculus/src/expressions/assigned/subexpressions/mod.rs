use std::fmt::Display;

use path_lib::{obj_at_path::{ObjAtPath, OwnedObjAtPath}};
use path_lib_proc_macros::generate_parent_of_children_trait;
use proof_calculus::utils::traits::fast_ord::{FastOrd, fastcmp_for_sorted_slices};

use crate::expressions::assigned::{TblExpression, compound::CompoundTblExpression, subexpressions::immediate::ImmediateTblSubexpressionInExpressionPath};

pub mod immediate;
pub mod iterators;

#[derive(Clone,PartialEq,Eq,Hash,Debug,Default)]
pub struct TblSubexpressionInExpressionPath(pub Vec<ImmediateTblSubexpressionInExpressionPath>);
// impl TblSubexpressionInExpressionPath {
//     pub fn predecessor(&self) -> Option<Self> {
//         if self.0.len() == 0 { None } else {
//             let slice = &self.0[0..self.0.len() - 1];
//             Some(Self(slice.iter().cloned().collect()))
//         }
//     }
// }
impl FastOrd for TblSubexpressionInExpressionPath {
    fn fast_cmp(&self, other: &Self) -> std::cmp::Ordering
        { fastcmp_for_sorted_slices(&self.0, &other.0) }
}
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
    impl From<ImmediateTblSubexpressionInExpressionPath> for TblSubexpressionInExpressionPath {
        fn from(value: ImmediateTblSubexpressionInExpressionPath) -> Self { vec![value].into() }
    }
    impl From<(ImmediateTblSubexpressionInExpressionPath,ImmediateTblSubexpressionInExpressionPath)> for TblSubexpressionInExpressionPath {
        fn from(value: (ImmediateTblSubexpressionInExpressionPath,ImmediateTblSubexpressionInExpressionPath)) -> Self { vec![value.0,value.1].into() }
    }
    impl From<Vec<ImmediateTblSubexpressionInExpressionPath>> for TblSubexpressionInExpressionPath {
        fn from(value: Vec<ImmediateTblSubexpressionInExpressionPath>) -> Self { Self(value) }
    }

    impl From<(TblSubexpressionInExpressionPath,ImmediateTblSubexpressionInExpressionPath)> for TblSubexpressionInExpressionPath {
        fn from(mut value: (TblSubexpressionInExpressionPath,ImmediateTblSubexpressionInExpressionPath)) -> Self {
            value.0.0.push(value.1);
            value.0
        }
    }
    impl From<(ImmediateTblSubexpressionInExpressionPath,TblSubexpressionInExpressionPath)> for TblSubexpressionInExpressionPath {
        fn from(mut value: (ImmediateTblSubexpressionInExpressionPath,TblSubexpressionInExpressionPath)) -> Self {
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
    // use crate::expressions::assigned::atomic::AtomicTblExpression;

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
