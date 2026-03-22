use std::{collections::HashSet};

use crate::expressions::Expression;

/// Every [Proposition] within Tuple-based Logic is simply an [Expression] whose truth value is to be considered
pub type Proposition = Expression;
pub type PropositionSet = HashSet<Proposition>;
