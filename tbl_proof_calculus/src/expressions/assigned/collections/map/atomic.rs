use std::collections::{HashMap, HashSet};

use crate::expressions::assigned::{ArcTblExpression, collections::tracker::TblExpressionTracker};

pub struct AtomicTblExpressionMap<T> {
    values: HashMap<ArcTblExpression,T>,
    tracker: TblExpressionTracker<ArcTblExpression>
}
