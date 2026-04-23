use std::collections::{HashMap, HashSet};

use crate::expressions::assigned::{ArcTblExpression, binding::binders::TblExpressionBinder};

pub struct AtomicTblExpressionMap<T> {
    values: HashMap<ArcTblExpression,T>,
    tracker: TblExpressionBinder<ArcTblExpression>
}
