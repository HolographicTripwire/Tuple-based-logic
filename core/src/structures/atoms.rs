use tbl_structures::expressions::{Expression, atomic::AtomicExpression};

/// Atoms which are built in to Tuple-Based Logic, and will appear in all axiomatic systems in Tuple-Based Logic
#[derive(Clone,Sequence)]
pub enum StandardAtoms {
    // Deduction
    Conjunction,
    Implication,
    UniversalQuantifier,
    // Identity
    Identity,
    // Contradiction
    Negation,
    // Verbatim
    Verbatim,
    Concatenate,
    Atomic,
}

impl Into<AtomicExpression> for StandardAtoms {
    /// Assigns each built in atom a unique atom id
    fn into(self) -> AtomicExpression {
        AtomicExpression(match self {
            // Deduction
            StandardAtoms::Conjunction => 0,
            StandardAtoms::UniversalQuantifier => 1,
            StandardAtoms::Implication => 2,
            // Contradiction
            StandardAtoms::Negation => 3,
            // Identity
            StandardAtoms::Identity => 4,
            // Verbatim
            StandardAtoms::Verbatim => 5,
            StandardAtoms::Concatenate => 6,
            StandardAtoms::Atomic => 7,
        })
    }
}

impl Into<AtomicExpression> for BuiltInAtom {
    fn into(self) -> AtomicExpression
        { AtomicExpression::from(self.into()) }
}
impl Into<Expression> for BuiltInAtom {
    fn into(self) -> Expression
        { Expression::from(self.into()) }
}


#[cfg(tests)]
mod tests {
    use enum_iterator::all;

    #[test]
    fn test_differentiation_of_builtins() {
        let builtins  = all::<BuiltInAtom>().collect::<Vec<_>>();
        for (i, ix) in builtins.iter().enumerate() {
            for (j, jx) in builtins.iter().enumerate() {
                let ia: AtomicExpression = (*ix).clone().into();
                let ja: AtomicExpression = (*jx).clone().into();
                if i==j { assert_eq!(ia,ja) }
                else { assert_ne!(ia,ja) }
            }
        }
    }
}
