use crate::structures::expressions::{patterns::{components::ExprPatternComponent, special_case::ExprPatternPair, ExprPattern}, raw::RawExpressionStyle, SpecialCase, SpecialCases};

pub(super) fn symbol_atom<'a>(atom_id: usize, after: &str, style: &RawExpressionStyle) -> ExprPatternPair 
    { symbol(&style.atom_style().to_id(atom_id), after) }
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

pub(super) fn prefix_function<'a>(function_head: &str, function_prefix: &str, style: &RawExpressionStyle) -> ExprPatternPair
    { prefix_function_inner(function_head.to_string(), function_prefix.to_string(), style) }
fn prefix_function_inner<'a>(function_head: String, function_prefix: String, style: &RawExpressionStyle) -> ExprPatternPair {
    ExprPatternPair::new(
        ExprPattern::new([
            ExprPatternComponent::new_const(style.tuple_opener()),
            ExprPatternComponent::new_const(&function_head),
            ExprPatternComponent::new_const(style.delimiter()),
            ExprPatternComponent::new_vars("A", style.delimiter(), "B"),
            ExprPatternComponent::new_const(style.tuple_closer())
        ]),
        ExprPattern::new([
            ExprPatternComponent::new_const(&function_prefix),
            ExprPatternComponent::new_const(style.tuple_opener()),
            ExprPatternComponent::new_vars("A", style.delimiter(), "B"),
            ExprPatternComponent::new_const(style.tuple_closer()),
        ])
    )
}

pub(super) fn infix_function<'a>(function_head: &str, function_infix: &str, style: &RawExpressionStyle) -> ExprPatternPair
    { infix_function_inner(function_head.to_string(), function_infix.to_string(), style) }
fn infix_function_inner<'a>(function_head: String, function_infix: String, style: &RawExpressionStyle) -> ExprPatternPair {
    ExprPatternPair::new(
        ExprPattern::new([
            ExprPatternComponent::new_const(style.tuple_opener()),
            ExprPatternComponent::new_const(&function_head),
            ExprPatternComponent::new_const(style.delimiter()),
            ExprPatternComponent::new_vars("A", style.delimiter(), "B"),
            ExprPatternComponent::new_const(style.tuple_closer())
        ]),
        ExprPattern::new([
            ExprPatternComponent::new_const(style.tuple_opener()),
            ExprPatternComponent::new_vars("A", &function_infix, "B"),
            ExprPatternComponent::new_const(style.tuple_closer()),
        ])
    )
}

pub(super) fn postfix_function<'a>(function_head: &str, function_postfix: &str, style: &RawExpressionStyle) -> ExprPatternPair
    { postfix_function_inner(function_head.to_string(), function_postfix.to_string(), style) }
fn postfix_function_inner<'a>(function_head: String, function_postfix: String, style: &RawExpressionStyle) -> ExprPatternPair {
    ExprPatternPair::new(
        ExprPattern::new([
            ExprPatternComponent::new_const(style.tuple_opener()),
            ExprPatternComponent::new_const(&function_head),
            ExprPatternComponent::new_const(style.delimiter()),
            ExprPatternComponent::new_vars("A", style.delimiter(), "B"),
            ExprPatternComponent::new_const(style.tuple_closer())
        ]),
        ExprPattern::new([
            ExprPatternComponent::new_const(style.tuple_opener()),
            ExprPatternComponent::new_vars("A", style.delimiter(), "B"),
            ExprPatternComponent::new_const(style.tuple_closer()),
            ExprPatternComponent::new_const(&function_postfix),
        ])
    )
}
