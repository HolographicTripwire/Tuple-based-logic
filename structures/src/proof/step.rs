#[derive(Clone)]
/// Identifies a particular step iwthin a [`Proof`], and can be given to such a [`Proof`] to retreive the [`SubProof`] at that step
pub struct ProofStep(pub Vec<usize>);

impl ProofStep {
    /// Create a new error, which is located at the current step of the proof
    pub fn here() -> Self { Self(vec![]) }
    /// Create a new error, located at a given substep of the current step of the proof.
    pub fn at_substep(step: usize) -> Self { Self(vec![step]) }

    /// Pushes a new step to the list of steps.
    /// [`ProofStep`] behaves like a queue, so the provided step will go to the back of the queue
    pub fn push(&mut self, step: usize) {self.0.insert(0,step)}
    pub fn pop(&mut self) -> Option<usize> { self.0.pop() }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_here() {
        let step = ProofStep::here();
        assert_eq!(step.0, vec![])
    }

    #[test]
    fn test_at_substep() {
        let step = ProofStep::at_substep(1);
        assert_eq!(step.0, vec![1])
    }

    #[test]
    fn test_push() {
        let mut step = ProofStep::at_substep(1);
        step.push(2);
        assert_eq!(step.0, vec![2,1])
    }

    #[test]
    fn test_pop() {
        let mut step = ProofStep::at_substep(1);
        step.push(2);
        assert_eq!(step.pop(), Some(1))
    }
}
