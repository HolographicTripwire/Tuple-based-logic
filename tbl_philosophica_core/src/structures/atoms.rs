use enum_iterator::Sequence;
use tbl_proof_calculus::expressions::assigned::{TblExpression, atomic::AtomicTblExpression, compound::CompoundTblExpression};

/// Atoms which are built in to Tuple-Based Logic, and will appear in all axiomatic systems in Tuple-Based Logic
#[derive(Clone,Sequence)]
pub enum PhilosophicaInferenceAtoms {
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

impl Into<AtomicTblExpression> for PhilosophicaInferenceAtoms {
    /// Assigns each built in atom a unique atom id
    fn into(self) -> AtomicTblExpression {
        AtomicTblExpression(match self {
            // Deduction
            PhilosophicaInferenceAtoms::Conjunction => 0,
            PhilosophicaInferenceAtoms::UniversalQuantifier => 1,
            PhilosophicaInferenceAtoms::Implication => 2,
            // Contradiction
            PhilosophicaInferenceAtoms::Negation => 3,
            // Identity
            PhilosophicaInferenceAtoms::Identity => 4,
            // Verbatim
            PhilosophicaInferenceAtoms::Verbatim => 5,
            PhilosophicaInferenceAtoms::Concatenate => 6,
            PhilosophicaInferenceAtoms::Atomic => 7,
        })
    }
}

impl <C: CompoundTblExpression> Into<TblExpression<C>> for PhilosophicaInferenceAtoms {
    fn into(self) -> TblExpression<C>
        { AtomicTblExpression::from(self.into()).into() }
}


#[cfg(test)]
mod tests {
    use enum_iterator::all;
    use tbl_proof_calculus::expressions::assigned::atomic::AtomicTblExpression;

    use crate::structures::atoms::PhilosophicaInferenceAtoms;

    #[test]
    fn test_differentiation_of_builtins() {
        let builtins  = all::<PhilosophicaInferenceAtoms>().collect::<Vec<_>>();
        for (i, ix) in builtins.iter().enumerate() {
            for (j, jx) in builtins.iter().enumerate() {
                let ia: AtomicTblExpression = (*ix).clone().into();
                let ja: AtomicTblExpression = (*jx).clone().into();
                if i==j { assert_eq!(ia,ja) }
                else { assert_ne!(ia,ja) }
            }
        }
    }
}
