use std::collections::{HashMap, HashSet};

use crate::expressions::assigned::{ArcTblExpression, collections::tracker::TblExpressionBinder};

pub struct AtomicTblExpressionMap<T> {
    values: HashMap<ArcTblExpression,T>,
    tracker: TblExpressionBinder<ArcTblExpression>
}
