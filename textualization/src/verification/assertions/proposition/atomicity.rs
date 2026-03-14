mod check {
    pub fn format_proposition_atomicity_check_error(err: PropositionAtomicityCheckError) -> String {
        format!("Proposition at {path} has wrong atomicity (expected {atomicity_expected}; found {atomicity_actual})",
            path=err.proposition.path(),
            atomicity_expected=stringify_atomicity(err.expected_atomicity),
            atomicity_actual=stringify_atomicity(err.proposition.obj().as_atom().is_ok())
        )
    }
}

mod equality {
    pub fn format_proposition_atomicity_equality_error(err: PropositionAtomicityEqualityError) -> String {
        format!("Proposition atomicities expected to all be equal, but weren't; {atomicities}",
            atomicities = itertools::join(err.propositions.iter().map(|o|
                o.path().to_string()
                + " -> " +
                stringify_atomicity(o.obj().as_atom().is_ok())
            ),", ")
        )
    }

    pub fn format_fixed_length_proposition_atomicity_equality_error<const N: usize>(err: FixedLengthPropositionAtomicityEqualityError<N>) -> String {
        format!("Proposition atomicities expected to all be equal, but weren't; {atomicities}",
            atomicities = itertools::join(err.propositions.iter().map(|o|
                o.path().to_string()
                + " -> " +
                stringify_atomicity(o.obj().as_atom().is_ok())
            ),", ")
        )
    }
}

mod inequality {

pub fn format_proposition_atomicity_inequality_error(err: PropositionAtomicityInequalityError) -> String {
    format!("Atomicity of propositions {prop1} and {prop2} expected to be inequal, but both were {value}",
                prop1 = err.prop1.path(),
                prop2 = err.prop2.path(),
                value = err.prop1.obj().as_atom().is_ok()
            )
    }
}
