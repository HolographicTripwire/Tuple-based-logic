use crate::proof::subproof_path::{AtomicSubproofPath, SubproofPath};

#[derive(Clone,PartialEq,Eq,Debug)]
pub struct ErrorInProof<E: Clone>(SubproofPath,E);

/// An error that is located at a particular step of a proof
/// This can even include substeps of substeps
impl <E: Clone> ErrorInProof<E> {
    /// Create a new error, which is located at the current step of the proof
    pub fn here(err: E) -> Self { Self(SubproofPath::empty(),err) }
    /// Create a new error, located at a given substep of the current step of the proof.
    pub fn at_substep(step: usize, err: E) -> Self { Self(SubproofPath::new([step]),err) }

    /// Add a new step to this error and return self
    /// This should be used for 
    pub fn push_step(&mut self, step: usize) -> &Self { self.0.0.prepend(step); self }
    /// Add a new step to this error and return self
    /// This should be used for 
    pub fn pop_step(&mut self) -> Option<AtomicSubproofPath> { self.0.0.pop() }

    // Getters and setters
    /// Get the location in the proof that this error is located at
    /// For instance, an error at step 1.2.1 would return a Vec containing 1, 2, 1, in that order
    pub fn location(&self) -> &SubproofPath { &self.0 }
    pub fn err(&self) -> &E { &self.1 }
}

pub enum ResultInProof<T,E: Clone> {
    Ok(T), Err(E),
    ErrNest(ErrorInProof<E>)
}

impl <T: Clone, E: Clone> ResultInProof<T,E> {
    /// Turn this ResultInProof instance into a Result instance, while still retaining information about the location of the error
    /// ### Accepts:
    /// - The step within the proof that we are at when this ResultInProof object gets resolved
    /// ### Returns:
    /// - Ok(T) if this is an Ok result
    /// - Err(ErrorInProof) if this is an Err (Error or ErrNest) result
    pub fn resolve(self, step: usize) -> Result<T,ErrorInProof<E>> { match self {
        ResultInProof::Ok(ok) => Ok(ok.clone()),
        ResultInProof::Err(err) => Result::Err(ErrorInProof::at_substep(step,err.clone())),
        ResultInProof::ErrNest(mut error_in_proof) => Result::Err(error_in_proof.push_step(step).clone()),
    }}
}
impl <T: Clone, E: Clone> From<Result<T,E>> for ResultInProof<T,E> {
    fn from(value: Result<T,E>) -> Self { match value {
        Ok(o) => Self::Ok(o),
        Err(e) => Self::Err(e),
    }}
}
impl <T: Clone, E: Clone> From<Result<T,ErrorInProof<E>>> for ResultInProof<T,E> {
    fn from(value: Result<T, ErrorInProof<E>>) -> Self { match value {
        Ok(o) => Self::Ok(o),
        Err(e) => Self::ErrNest(e),
    }}
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Clone,PartialEq,Eq,Debug)]
    enum TestError { Error }

    #[test]
    fn test_here() {
        let step = ErrorInProof::here(TestError::Error);
        assert_eq!(step.location().0.paths(), &vec![]);
        assert_eq!(step.err(), &TestError::Error);
    }

    #[test]
    fn test_at_substep() {
        let step = ErrorInProof::at_substep(1,TestError::Error);
        assert_eq!(step.location().0.paths(), &vec![(1 as usize).into()]);
        assert_eq!(step.err(), &TestError::Error);
    }

    #[test]
    fn test_push() {
        let mut step = ErrorInProof::at_substep(1, TestError::Error);
        step.push_step(2);
        assert_eq!(step.location().0.paths(), &vec![(2 as usize).into(),(1 as usize).into()]);
        assert_eq!(step.err(), &TestError::Error);
    }

    #[test]
    fn test_pop() {
        let mut step = ErrorInProof::at_substep(1,TestError::Error);
        step.push_step(2);
        assert_eq!(step.pop_step(), Some((1 as usize).into()));
        assert_eq!(step.err(), &TestError::Error);
    }
}
