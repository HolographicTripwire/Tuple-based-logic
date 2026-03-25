pub mod inference;
pub mod composite;
pub mod at_path_enum;
mod in_proof;
pub mod error;


#[cfg(test)]
mod tests {
    use crate::sequential_proofs::in_proof::ProofInProofPath;    

    #[test]
    fn test_getters() {
        let step = ProofInProofPath::default();
        assert_eq!(step.0, vec![])
    }
}
