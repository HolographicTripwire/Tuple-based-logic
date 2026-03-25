use crate::inference::Inference;

pub trait InferenceRule<Proposition>: 'static + Clone + PartialEq {
}


pub trait ProgressableInferenceRule<Proposition> {

}

pub trait RegressableInferenceRule<Proposition> {
    fn regress(expr: Expression);
}

pub trait VerifiableInferenceRule<Proposition> {
    type Err: Clone;

    fn verify(rule: &Inference<Proposition,Self>) -> Result<(),Self::Err>;
}
