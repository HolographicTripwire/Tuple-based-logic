use crate::{proofs::{inferences::InferenceRule, sequential::SequentialProof}, propositions::types::assigned::{ParentOfAssumptions, Proposition, collections::sets::PropSet1O}};

/// Check that all of the premises of a given [Proof] are contained within some [PropositionSet]
/// Used to check the "grounding" of a proof - that is, are all of the proof's premises assumed to be true? If they are, we can trust the proof's conclusions
pub fn verify_proof_grounding<'a, P: Proposition, PS: PropSet1O<P>, Rule: InferenceRule<P>>(proof: &'a SequentialProof<P,Rule>, assumptions: &PS) -> Result<(),&'a P> {
    proof_grounding_helper(proof, assumptions)
        .collect()
}

pub fn get_proof_grounding_errors<'a, P: Proposition, PS: PropSet1O<P>, Rule: InferenceRule<P>>(proof: &'a SequentialProof<P,Rule>, assumptions: &PS) -> impl Iterator<Item = &'a P> {
    proof_grounding_helper(proof, assumptions)
        .filter_map(|expr| match expr {
            Ok(_) => None,
            Err(e) => Some(e),
        })
}

fn proof_grounding_helper<'a, P: Proposition, PS: PropSet1O<P>, Rule: InferenceRule<P>>(proof: &'a SequentialProof<P,Rule>, assumptions: &PS) -> impl Iterator<Item = Result<(),&'a P>> {
    proof.get_assumptions()
        .into_iter()
        .map(|premise| if assumptions.contains(premise) { Ok(()) } else { Err(premise) })
}
