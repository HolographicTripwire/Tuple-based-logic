mod check {
    pub fn format_proposition_value_check_error(err: PropositionValueCheckError, style: ExpressionStyle) -> String {
        format!("Proposition at {path} has wrong value (expected {value_expected}; found {value_actual})",
            path=err.proposition.path(),
            value_expected=style.stringify(&err.expected_value),
            value_actual=style.stringify(err.proposition.obj)
        )
    }
}

mod equality {
    pub fn format_proposition_value_equality_error(err: PropositionValueEqualityError, style: PropositionStyle) -> String {
        format!("Proposition values expected to all be equal, but weren't; {atomicities}",
            atomicities = err.propositions.iter().map(|o|
                o.path().to_string()
                + " -> " +
                &style.stringify(o.obj)
            ).collect_vec().join(", ")
        )
    }
    pub fn format_fixed_length_proposition_value_equality_error<const N: usize>(err: FixedLengthPropositionValueEqualityError<N>, style: PropositionStyle) -> String {
        format!("Proposition values expected to all be equal, but weren't; {atomicities}",
            atomicities = err.propositions.iter().map(|o|
                o.path().to_string()
                + " -> " +
                &style.stringify(o.obj)
            ).collect_vec().join(", ")
        )
    }
}

mod inequality {
    pub fn format_proposition_value_inequality_error(err: PropositionValueInequalityError, style: PropositionStyle) -> String {
        format!("Proposition values expected to all be inequal, but weren't; {values}",
            values = err.propositions.iter().map(|o|
                o.path().to_string()
                + " -> " +
                &style.stringify(o.obj)
            ).collect_vec().join(", ")
        )
    }
    pub fn format_fixed_length_proposition_value_inequality_error<const N: usize>(err: FixedLengthPropositionValueInequalityError<N>, style: PropositionStyle) -> String {
        format!("Proposition lengths expected to all be equal, but weren't; {atomicities}",
            atomicities = err.propositions.iter().map(|o|
                o.path().to_string()
                + " -> " +
                &style.stringify(o.obj)
            ).collect_vec().join(", ")
        )
    }
}
