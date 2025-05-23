use super::ProofStep;

pub struct ErrorInProof<E: Clone>(ProofStep,E);

/// An error that is located at a particular step of a proof
/// This can even include substeps of substeps
impl <E: Clone> ErrorInProof<E> {
    /// Create a new error, which is located at the current step of the proof
    pub fn here(err: E) -> Self { Self(ProofStep::here(),err) }
    /// Create a new error, located at a given substep of the current step of the proof.
    pub fn at_substep(step: usize, err: E) -> Self { Self(ProofStep::at_substep(step),err) }

    /// Add a new step to this error and return self
    /// This should be used for 
    pub fn push_step(mut self, step: usize) -> Self { self.0.push(step); self }
    /// Add a new step to this error and return self
    /// This should be used for 
    pub fn pop_step(mut self) -> Option<usize> { self.0.pop() }

    // Getters and setters
    /// Get the location in the proof that this error is located at
    /// For instance, an error at step 1.2.1 would return a Vec containing 1, 2, 1, in that order
    pub fn location(&self) -> &ProofStep { &self.0 }
    pub fn err(&self) -> &E { &self.1 }
}

pub enum ResultInProof<O,E: Clone> {
    Ok(O), Err(E),
    ErrNest(ErrorInProof<E>)
}

impl <O: Clone, E: Clone> ResultInProof<O,E> {
    pub fn resolve(self, step: usize) -> Result<O,ErrorInProof<E>> { match self {
        ResultInProof::Ok(ok) => Ok(ok.clone()),
        ResultInProof::Err(err) => Result::Err(ErrorInProof::at_substep(step,err.clone())),
        ResultInProof::ErrNest(error_in_proof) => Result::Err(error_in_proof.push_step(step)),
    }}
}
impl <O: Clone, E: Clone> From<Result<O,E>> for ResultInProof<O,E> {
    fn from(value: Result<O,E>) -> Self { match value {
        Ok(o) => Self::Ok(o),
        Err(e) => Self::Err(e),
    }}
}
impl <O: Clone, E: Clone> From<Result<O,ErrorInProof<E>>> for ResultInProof<O,E> {
    fn from(value: Result<O, ErrorInProof<E>>) -> Self { match value {
        Ok(o) => Self::Ok(o),
        Err(e) => Self::ErrNest(e),
    }}
}
