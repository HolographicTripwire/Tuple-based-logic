pub fn format_assumption_count_check_error<Rule: InferenceRule>(err: AssumptionCountCheckError<Rule>) -> String {
    format!("Inference has wrong number of assumptions (expected {expected_count}; found {actual_count}",
        expected_count=err.expected_count,
        actual_count=err.get_actual_count()
    )
}

pub fn format_explicit_conclusion_count_check_error<Rule: InferenceRule>(err: ExplicitConclusionCountCheckError<Rule>) -> String {
    format!("Inference has wrong number of explicit conclusions (expected {expected_count}; found {actual_count}",
        expected_count = err.expected_count,
        actual_count = err.inference.assumptions.len()
    )
}