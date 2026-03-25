
use std::fmt::Display;

use path_lib::{obj_at_path::{ObjAtPath, OwnedObjAtPath}};

use crate::expressions::Proposition;

// impl <'a> Into<ExpressionInInference<'a>> for PropositionInProofStep<'a> {
//     fn into(self) -> ExpressionInInference<'a> {
//         let (obj, path) = self.0.into_obj_and_path();
//         ExpressionInInference::from_inner(obj, ExpressionInInferencePath {
//             proposition_path: path,
//             subexpression_path: ExpressionInExpressionPath::default()
//         })
//     }
// }
