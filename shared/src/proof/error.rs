pub struct ErrorInProof<E: Clone>(Vec<usize>,E);

impl <E: Clone> ErrorInProof<E> {
    pub fn new_at_step(step: usize, err: E) -> Self { Self(vec![step],err) }
    pub fn new(err: E) -> Self { Self(vec![],err) }

    pub fn add_step(&self, step: usize) -> Self {
        let mut steps = self.0.clone();
        steps.insert(step,0);
        Self(steps,self.1.clone())
    }

    pub fn location(&self) -> &Vec<usize> { &self.0 }
    pub fn err(&self) -> &E { &self.1 }
}

pub enum ResultInProof<O,E: Clone> {
    Ok(O),
    Err(E),
    ErrNest(ErrorInProof<E>)
}

impl <O: Clone, E: Clone> ResultInProof<O,E> {
    pub fn resolve(&self, step: usize) -> Result<O,ErrorInProof<E>> {
        match self {
            ResultInProof::Ok(ok) => Ok(ok.clone()),
            ResultInProof::Err(err) => Result::Err(ErrorInProof::new_at_step(step,err.clone())),
            ResultInProof::ErrNest(error_in_proof) => Result::Err(error_in_proof.add_step(step)),
        }
    }
}

impl <O: Clone, E: Clone> From<Result<O,E>> for ResultInProof<O,E> {
    fn from(value: Result<O,E>) -> Self {
        match value {
            Ok(o) => Self::Ok(o),
            Err(e) => Self::Err(e),
        }
    }
}

impl <O: Clone, E: Clone> From<Result<O,ErrorInProof<E>>> for ResultInProof<O,E> {
    fn from(value: Result<O, ErrorInProof<E>>) -> Self {
        match value {
            Ok(o) => Self::Ok(o),
            Err(e) => Self::ErrNest(e),
        }
    }
}
