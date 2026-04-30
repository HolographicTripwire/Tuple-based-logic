use std::fmt::Display;

use proof_calculus::utils::traits::fast_ord::FastOrd;


#[derive(Clone,Copy,PartialEq,Eq,Hash,Debug,PartialOrd,Ord)]
pub struct ImmediateTblSubexpressionInExpressionPath(pub usize);
impl From<usize> for ImmediateTblSubexpressionInExpressionPath {
    fn from(value: usize) -> Self { Self(value) }
}
impl FastOrd for ImmediateTblSubexpressionInExpressionPath {
    fn fast_cmp(&self, other: &Self) -> std::cmp::Ordering { self.cmp(other) }
}

impl Display for ImmediateTblSubexpressionInExpressionPath {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
        { write!(f, "{}", self.0) }
}

