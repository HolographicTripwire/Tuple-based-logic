pub mod check {
    pub fn format_expression_length_check_error(err: ExpressionLengthCheckError) -> String {
        let expression = err.expression.obj();
        format!("Expression at {path} has wrong length (expected {length_expected}; found {length_actual})",
            path=err.expression.path(),
            length_expected=stringify_length(expression),
            length_actual=stringify_length(expression)
        )
    }
}

pub mod equality {
    pub fn format_expression_length_equality_error(err: ExpressionLengthEqualityError) -> String {
        format!("Expression lengths expected to all be equal, but weren't; {atomicities}",
            atomicities = err.expressions.iter().map(|o|
                o.path().to_string()
                + " -> " +
                &stringify_length(o.obj())
            ).collect::<Vec<_>>().join(", ")
        )
    }
    
    pub fn format_fixed_length_expression_length_equality_error<const N: usize>(err: FixedLengthExpressionLengthEqualityError<N>) -> String {
        format!("Expression lengths expected to all be equal, but weren't; {atomicities}",
            atomicities = err.expressions.iter().map(|o|
                o.path().to_string()
                + " -> " +
                &stringify_length(o.obj())
            ).collect::<Vec<_>>().join(", ")
        )
    }
}

pub mod inequality {
    pub fn format_expression_length_inequality_error(err: ExpressionLengthInequalityError) -> String {
        format!("Expression lengths expected to all be inequal, but weren't; {lengths}",
            lengths = err.expressions.iter().map(|o|
                o.path().to_string()
                + " -> " +
                &stringify_length(o.obj())
            ).collect::<Vec<_>>().join(", ")
        )
    }
    
    pub fn format_fixed_length_expression_length_inequality_error<const N: usize>(err: FixedLengthExpressionLengthInequalityError<N>) -> String {
        format!("Expression lengths expected to all be equal, but weren't; {atomicities}",
            atomicities = err.expressions.iter().map(|o|
                o.path().to_string()
                + " -> " +
                &stringify_length(o.obj())
            ).collect::<Vec<_>>().join(", ")
        )
    }
}
