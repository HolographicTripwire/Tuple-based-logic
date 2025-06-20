use crate::structures::expressions::{patterns::{components::ExprPatternComponent, special_case::ExprPatternPair, ExprPattern}, raw::RawExpressionStyle};

pub(super) fn symbol_atom(atom_id: usize, after: &str, style: &RawExpressionStyle) -> ExprPatternPair 
    { symbol(&style.atom_style().to_id(atom_id), after) }
pub(super) fn symbol(before: &str, after: &str) -> ExprPatternPair {
    ExprPatternPair::new(
        ExprPattern::new([
            ExprPatternComponent::new_const(before)
        ]), ExprPattern::new([
            ExprPatternComponent::new_const(after)
        ])
    )
}

pub(super) fn prefix_function(input_head: &str, output_prefix: &str, style: &RawExpressionStyle) -> ExprPatternPair { allfix_function(
    input_head,
    &[output_prefix,style.tuple_opener()].concat(),
    style.delimiter(),
    style.tuple_closer(),
    style)
}

pub(super) fn infix_function(input_head: &str, output_infix: &str, style: &RawExpressionStyle) -> ExprPatternPair { allfix_function(
    input_head,
    style.tuple_opener(),
    output_infix,
    style.tuple_closer(),
    style)
}

pub(super) fn postfix_function(input_head: &str, output_postfix: &str, style: &RawExpressionStyle) -> ExprPatternPair { allfix_function(
    input_head,
    style.tuple_opener(),
    style.delimiter(),
    &[style.tuple_closer(),output_postfix].concat(),
    style)
}

pub(super) fn outfix_function(input_head: &str, output_left: &str, output_right: &str, style: &RawExpressionStyle) -> ExprPatternPair { allfix_function(
    input_head,
    output_left,
    style.delimiter(),
    output_right,
    style)
}

pub(super) fn allfix_function<'a>(input_head: &str, output_left: &str, output_infix: &str, output_right: &str, raw_style: &RawExpressionStyle) -> ExprPatternPair {
    ExprPatternPair::new(
        ExprPattern::new([
            ExprPatternComponent::new_const(raw_style.tuple_opener()),
            ExprPatternComponent::new_const(input_head),
            ExprPatternComponent::new_const(raw_style.delimiter()),
            ExprPatternComponent::new_vars("A", raw_style.delimiter(), "B"),
            ExprPatternComponent::new_const(raw_style.tuple_closer())
        ]),
        ExprPattern::new([
            ExprPatternComponent::new_const(output_left),
            ExprPatternComponent::new_vars("A", output_infix, "B"),
            ExprPatternComponent::new_const(output_right)
        ])
    )
}
