use tbl_structures::{inference::path::InferenceSubexpressionPath, propositions::Expression};


#[derive(Clone)]
pub enum ProofStepSpecificationError {
    WrongAssumptionCount(usize),
    WrongConclusionCount(usize),
    WrongLength(InferenceSubexpressionPath,usize),
    WrongValue(InferenceSubexpressionPath,Expression),
    MismatchedLengths(InferenceSubexpressionPath,InferenceSubexpressionPath),
    MismatchedValues(InferenceSubexpressionPath,InferenceSubexpressionPath)
}
