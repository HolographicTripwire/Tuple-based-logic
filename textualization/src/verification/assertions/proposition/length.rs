mod check {    
    pub fn format_proposition_length_check_error(err: PropositionLengthCheckError) -> String {
        let proposition = err.proposition.obj();
        format!("Proposition at {path} has wrong length (expected {length_expected}; found {length_actual})",
            path=err.proposition.path(),
            length_expected=stringify_length(proposition),
            length_actual=stringify_length(proposition)
        )
    }
}

mod equality {
    pub fn format_proposition_length_equality_error(err: PropositionLengthEqualityError) -> String {
        format!("Proposition lengths expected to all be equal, but weren't; {atomicities}",
            atomicities = err.propositions.iter().map(|o|
                o.path().to_string()
                + " -> " +
                &stringify_length(o.obj())
            ).collect::<Vec<_>>().join(", ")
        )
    }
    pub fn format_fixed_length_proposition_length_equality_error<const N: usize>(err: FixedLengthPropositionLengthEqualityError<N>) -> String {
        format!("Proposition lengths expected to all be equal, but weren't; {atomicities}",
            atomicities = err.propositions.iter().map(|o|
                o.path().to_string()
                + " -> " +
                &stringify_length(o.obj())
            ).collect::<Vec<_>>().join(", ")
        )
    }
}

mod inequality {
    pub fn format_proposition_length_inequality_error(err: PropositionLengthInequalityError) -> String {
        format!("Proposition lengths expected to all be inequal, but weren't; {lengths}",
            lengths = err.propositions.iter().map(|o|
                o.path().to_string()
                + " -> " +
                &stringify_length(o.obj())
            ).collect::<Vec<_>>().join(", ")
        )
    }
    pub fn format_fixed_length_proposition_length_inequality_error<const N: usize>(err: FixedLengthPropositionLengthInequalityError<N>) -> String {
        format!("Proposition lengths expected to all be equal, but weren't; {atomicities}",
            atomicities = err.propositions.iter().map(|o|
                o.path().to_string()
                + " -> " +
                &stringify_length(o.obj())
            ).collect::<Vec<_>>().join(", ")
        )
    }
}
