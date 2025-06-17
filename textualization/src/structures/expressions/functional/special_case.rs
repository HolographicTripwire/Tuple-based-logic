use crate::structures::expressions::{patterns::{components::ExprPatternComponent, special_case::ExprPatternPair, ExprPattern}, raw::RawExpressionControls, SpecialCase, SpecialCases};

pub(super) fn symbol_atom<'a>(atom_id: usize, after: &str, controls: &RawExpressionControls) -> ExprPatternPair 
    { symbol(&controls.atom_controls().to_id(atom_id), after) }
pub(super) fn symbol<'a>(before: &str, after: &str) -> ExprPatternPair
    { symbol_inner(before.to_string(), after.to_string()) }
fn symbol_inner<'a>(before: String, after: String) -> ExprPatternPair {
    ExprPatternPair::new(
        ExprPattern::new([
            ExprPatternComponent::new_const(&before)
        ]), ExprPattern::new([
            ExprPatternComponent::new_const(&after)
        ])
    )
}

pub(super) fn prefix_function<'a>(function_head: &str, function_prefix: &str, controls: &RawExpressionControls) -> ExprPatternPair
    { prefix_function_inner(function_head.to_string(), function_prefix.to_string(), controls) }
fn prefix_function_inner<'a>(function_head: String, function_prefix: String, controls: &RawExpressionControls) -> ExprPatternPair {
    ExprPatternPair::new(
        ExprPattern::new([
            ExprPatternComponent::new_const(controls.tuple_opener()),
            ExprPatternComponent::new_const(&function_head),
            ExprPatternComponent::new_const(controls.delimiter()),
            ExprPatternComponent::new_vars("A", controls.delimiter(), "B"),
            ExprPatternComponent::new_const(controls.tuple_closer())
        ]),
        ExprPattern::new([
            ExprPatternComponent::new_const(&function_prefix),
            ExprPatternComponent::new_const(controls.tuple_opener()),
            ExprPatternComponent::new_vars("A", controls.delimiter(), "B"),
            ExprPatternComponent::new_const(controls.tuple_closer()),
        ])
    )
}

pub(super) fn infix_function<'a>(function_head: &str, function_infix: &str, controls: &RawExpressionControls) -> ExprPatternPair
    { infix_function_inner(function_head.to_string(), function_infix.to_string(), controls) }
fn infix_function_inner<'a>(function_head: String, function_infix: String, controls: &RawExpressionControls) -> ExprPatternPair {
    ExprPatternPair::new(
        ExprPattern::new([
            ExprPatternComponent::new_const(controls.tuple_opener()),
            ExprPatternComponent::new_const(&function_head),
            ExprPatternComponent::new_const(controls.delimiter()),
            ExprPatternComponent::new_vars("A", controls.delimiter(), "B"),
            ExprPatternComponent::new_const(controls.tuple_closer())
        ]),
        ExprPattern::new([
            ExprPatternComponent::new_const(controls.tuple_opener()),
            ExprPatternComponent::new_vars("A", &function_infix, "B"),
            ExprPatternComponent::new_const(controls.tuple_closer()),
        ])
    )
}

pub(super) fn postfix_function<'a>(function_head: &str, function_postfix: &str, controls: &RawExpressionControls) -> ExprPatternPair
    { postfix_function_inner(function_head.to_string(), function_postfix.to_string(), controls) }
fn postfix_function_inner<'a>(function_head: String, function_postfix: String, controls: &RawExpressionControls) -> ExprPatternPair {
    ExprPatternPair::new(
        ExprPattern::new([
            ExprPatternComponent::new_const(controls.tuple_opener()),
            ExprPatternComponent::new_const(&function_head),
            ExprPatternComponent::new_const(controls.delimiter()),
            ExprPatternComponent::new_vars("A", controls.delimiter(), "B"),
            ExprPatternComponent::new_const(controls.tuple_closer())
        ]),
        ExprPattern::new([
            ExprPatternComponent::new_const(controls.tuple_opener()),
            ExprPatternComponent::new_vars("A", controls.delimiter(), "B"),
            ExprPatternComponent::new_const(controls.tuple_closer()),
            ExprPatternComponent::new_const(&function_postfix),
        ])
    )
}
