mod check {
    pub fn format_expression_atomicity_check_error(err: ExpressionAtomicityCheckError) -> String {
        format!("Expression at {path} has wrong atomicity (expected {atomicity_expected}; found {atomicity_actual})",
            path=err.expression.path(),
            atomicity_expected=stringify_atomicity(err.expected_atomicity),
            atomicity_actual=stringify_atomicity(err.expression.obj().as_atom().is_ok())
        )
    }
}

mod equality {
    pub fn format_expression_atomicity_equality_error(err: ExpressionAtomicityEqualityError) -> String {
        format!("Expression atomicities expected to all be equal, but weren't; {atomicities}",
            atomicities = itertools::join(err.expressions.iter().map(|o|
                o.path().to_string()
                + " -> " +
                stringify_atomicity(o.obj().as_atom().is_ok())
            ),", ")
        )
    }

    pub fn format_fixed_length_expression_atomicity_equality_error<const N: usize>(err: FixedLengthExpressionAtomicityEqualityError<N>) -> String {
        format!("Expression atomicities expected to all be equal, but weren't; {atomicities}",
            atomicities = itertools::join(err.expressions.iter().map(|o|
                o.path().to_string()
                + " -> " +
                stringify_atomicity(o.obj().as_atom().is_ok())
            ),", ")
        )
    }
}

mod inequality {
    pub fn format_expression_atomicity_inequality_error(err: ExpressionAtomicityInequalityError) -> String {
        format!("Atomicity of expressions {expr1} and {expr2} expected to be inequal, but both were {value}",
                expr1 = err.expr1.path(),
                expr2 = err.expr2.path(),
                value = err.expr1.obj().as_atom().is_ok()
            )
    }
}
