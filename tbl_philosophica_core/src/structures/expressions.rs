use tbl_proof_calculus::expressions::assigned::{TblExpression, compound::CompoundTblExpression};

use crate::structures::atoms::PhilosophicaInferenceAtoms;

/// Get the expression which is negated by this expression
/// Returns Some(&negated_expression) if this expression is the negation of some negated_expression, otherwise returns None
/// For example, get_negated((¬,(¬,P))) = (¬,P)
pub fn get_negated<C: CompoundTblExpression>(prop: &TblExpression<C>) -> Option<&TblExpression<C>> {
    // Try splitting this atom into two components. On failure, this expression is not a well-formed negation, so return None
    let Ok([negation_atom, negated_expression]) = prop.as_slice() else { return None; };
    // If the head of this expression is not a negation atom, this expression is not a well-formed negation, so return false
    if negation_atom != &PhilosophicaInferenceAtoms::Negation.into() { return None; }
    // We now know that the expression is a well-formed negation, so we return the expression being negated
    Some(negated_expression)
}

/// Check if this expression is the negation of another
pub fn is_negation_of<C1,C2>(prop1: &TblExpression<C1>, prop2: &TblExpression<C2>) -> bool where
C1: CompoundTblExpression + PartialEq<C2>,
C2: CompoundTblExpression + PartialEq<C1> 
    { if let Some(p) = get_negated(prop1) { p == prop2 } else { false } }

/// Get the number of negations that this proposition begins with
/// Note that a negation level is only counted if that level contains two terms - where one is the negation.
/// So, (¬,(¬,P)) counts as two, but (¬,(¬,P,Q)) and (¬,(¬)) only count as one
pub fn negation_level<C1: CompoundTblExpression>(prop: &TblExpression<C1>) -> usize {
    // Inductive case; if this expression negates something, then its negation level is the negated expressoin's negation level plus one
    if let Some(negated_expression) = get_negated(prop)
        { negation_level(negated_expression) + 1 }
    // Base case; if this expression doesn't negate anything than the negation level is zero
    else { 0 }
}

#[cfg(test)]
mod tests {
    use enum_iterator::cardinality;
    use tbl_proof_calculus::expressions::assigned::{TblExpression, compound::r#box::BoxCompoundTblExpression};

    use crate::structures::{atoms::PhilosophicaInferenceAtoms, expressions::get_negated};

    #[test]
    fn test_get_negated_on_non_negation() {
        let x: TblExpression<BoxCompoundTblExpression> = TblExpression::Atomic(cardinality::<PhilosophicaInferenceAtoms>().try_into().unwrap());
        assert_eq!(get_negated(&x), None)
    }

    #[test]
    fn test_get_negated_on_negation() {
        let neg: TblExpression<BoxCompoundTblExpression> = PhilosophicaInferenceAtoms::Negation.into();
        let x = TblExpression::Atomic(cardinality::<PhilosophicaInferenceAtoms>().try_into().unwrap());
        let neg_x = TblExpression::from(vec![neg,x.clone()]);
        assert_eq!(get_negated(&neg_x), Some(&x))
    }

    #[test]
    fn test_get_negated_on_double_negation() {
        let neg: TblExpression<BoxCompoundTblExpression> = PhilosophicaInferenceAtoms::Negation.into();
        let x = TblExpression::Atomic(cardinality::<PhilosophicaInferenceAtoms>().try_into().unwrap());
        let neg_x = TblExpression::from(vec![neg.clone(),x.clone()]);
        let neg_neg_x = TblExpression::from(vec![neg,neg_x.clone()]);
        assert_eq!(get_negated(&neg_neg_x), Some(&neg_x))
    }
}
