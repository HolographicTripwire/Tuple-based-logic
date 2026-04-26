use crate::expressions::unassigned::{UnassignedTblExpression, compound::UnassignedCompoundTblExpression};

struct NormalisedUnassignedTblExpression<C: UnassignedCompoundTblExpression>(UnassignedTblExpression<C>);
