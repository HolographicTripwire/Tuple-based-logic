pub struct ErrorInProof<E: Clone>(Vec<usize>,E);

impl <E: Clone> ErrorInProof<E> {
    pub fn new(step: usize, err: E) -> Self { Self(vec![step],err) }

    pub fn add_step(&self, step: usize) -> Self {
        let mut steps = self.0.clone();
        steps.insert(step,0);
        Self(steps,self.1.clone())
    }

    pub fn location(&self) -> &Vec<usize> { &self.0 }
    pub fn err(&self) -> &E { &self.1 }
}
