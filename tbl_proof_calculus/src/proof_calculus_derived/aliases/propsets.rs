use proof_calculus::propositions::types::assigned::collections::sets::implementations::hash::HashPropSet1O;

use crate::expressions::types::assigned::{TblExpression, compound::CompoundTblExpression};

pub type HashTblPropSet1O<C:CompoundTblExpression> = HashPropSet1O<TblExpression<C>>;
//pub type HashTblPropSet2O<C:CompoundTblExpression> = HashPropSet2O<TblExpression<C>>;

