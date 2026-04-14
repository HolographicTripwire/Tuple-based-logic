use std::collections::{HashMap, HashSet};

use crate::structures::expressions::{ArcTblExpression, atomic::AtomicTblExpression, collections::tracker::TblExpressionTracker, subexpressions::TblSubexpressionInExpressionPath};

pub struct AtomicTblExpressionMap<T> {
    values: HashMap<ArcTblExpression,T>,
    tracker: TblExpressionTracker<ArcTblExpression>
}
