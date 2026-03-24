
/// Check that all of the premises of a given [Proof] are contained within some [PropositionSet]
/// Used to check the "grounding" of a proof - that is, are all of the proof's premises assumed to be true? If they are, we can trust the proof's conclusions
pub fn verify_proof_grounding<'a, Rule: InferenceRule>(proof: &'a Proof<Rule>, assumptions: &TblPropSet) -> Result<(),&'a TblProposition> {
    proof_grounding_helper(proof, assumptions)
        .collect()
}

pub fn get_proof_grounding_errors<'a, Rule: InferenceRule>(proof: &'a Proof<Rule>, assumptions: &TblPropSet) -> impl Iterator<Item = &'a TblProposition> {
    proof_grounding_helper(proof, assumptions)
        .filter_map(|expr| expr.err())
}

fn proof_grounding_helper<'a, Rule: InferenceRule>(proof: &'a Proof<Rule>, assumptions: &TblPropSet) -> impl Iterator<Item = Result<(),&'a TblProposition>> {
    proof.get_assumptions()
        .into_iter()
        .map(|premise| if assumptions.contains(premise) { Ok(()) } else { Err(premise) })
}
