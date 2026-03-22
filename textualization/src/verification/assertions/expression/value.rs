pub mod check {
    pub fn format_expression_value_check_error(err: ExpressionValueCheckError, style: ExpressionStyle) -> String {
        format!("Expression at {path} has wrong value (expected {value_expected}; found {value_actual})",
            path=err.expression.path(),
            value_expected=style.stringify(&err.expected_value),
            value_actual=style.stringify(err.expression.obj)
        )
    }
}

pub mod equality {
    pub fn format_expression_value_equality_error(err: ExpressionValueEqualityError, style: ExpressionStyle) -> String {
        format!("Expression values expected to all be equal, but weren't; {atomicities}",
            atomicities = err.expressions.iter().map(|o|
                o.path().to_string()
                + " -> " +
                &style.stringify(o.obj)
            ).collect::<Vec<_>>().join(", ")
        )
    }
    pub fn format_fixed_length_expression_value_equality_error<const N: usize>(err: FixedLengthExpressionValueEqualityError<N>, style: ExpressionStyle) -> String {
        format!("Expression values expected to all be equal, but weren't; {atomicities}",
            atomicities = err.expressions.iter().map(|o|
                o.path().to_string()
                + " -> " +
                &style.stringify(o.obj)
            ).collect::<Vec<_>>().join(", ")
        )
    }
}

pub mod inequality {
    pub fn format_expression_value_inequality_error(err: ExpressionValueInequalityError, style: ExpressionStyle) -> String {
        format!("Proposition values expected to all be inequal, but weren't; {values}",
            values = err.expressions.iter().map(|o|
                o.path().to_string()
                + " -> " +
                &style.stringify(o.obj)
            ).collect::<Vec<_>>().join(", ")
        )
    }

    pub fn format_fixed_length_expression_value_inequality_error<const N: usize>(err: FixedLengthExpressionValueInequalityError<N>, style: ExpressionStyle) -> String {
        format!("Expression lengths expected to all be equal, but weren't; {atomicities}",
            atomicities = err.expressions.iter().map(|o|
                o.path().to_string()
                + " -> " +
                &style.stringify(o.obj)
            ).collect::<Vec<_>>().join(", ")
        )
    }
}
