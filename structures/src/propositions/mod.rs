use std::{collections::HashSet};

use crate::expressions::TblExpression;

/// Every [Proposition] within Tuple-based Logic is simply an [Expression] whose truth value is to be considered
pub type TblProposition = TblExpression;
pub type TblPropSet = HashSet<TblProposition>;
