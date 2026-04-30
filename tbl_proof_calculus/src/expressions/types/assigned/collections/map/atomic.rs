use std::collections::HashMap;

use crate::expressions::types::assigned::{ArcTblExpression, binding::binders::TblExpressionBinder};

pub struct AtomicTblExpressionMap<T> {
    values: HashMap<ArcTblExpression,T>,
    tracker: TblExpressionBinder<ArcTblExpression>
}
