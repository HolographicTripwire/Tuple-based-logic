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

pub(super) fn variadic_prefix_function(input_head: &str, output_prefix: &str, style: &RawExpressionStyle) -> ExprPatternPair { variadic_allfix_function(
    input_head,
    &[output_prefix,style.tuple_opener()].concat(),
    style.delimiter(),
    style.tuple_closer(),
    style)
}
pub(super) fn prefix_function(input_head: &str, arity: usize, output_prefix: &str, style: &RawExpressionStyle) -> ExprPatternPair { allfix_function(
    input_head,
    arity,
    &[output_prefix,style.tuple_opener()].concat(),
    style.delimiter(),
    style.tuple_closer(),
    style)
}


pub(super) fn variadic_infix_function(input_head: &str, output_infix: &str, style: &RawExpressionStyle) -> ExprPatternPair { variadic_allfix_function(
    input_head,
    style.tuple_opener(),
    output_infix,
    style.tuple_closer(),
    style)
}
pub(super) fn infix_function(input_head: &str, arity: usize, output_infix: &str, style: &RawExpressionStyle) -> ExprPatternPair { allfix_function(
    input_head,
    arity,
    style.tuple_opener(),
    output_infix,
    style.tuple_closer(),
    style)
}

pub(super) fn variadic_postfix_function(input_head: &str, output_postfix: &str, style: &RawExpressionStyle) -> ExprPatternPair { variadic_allfix_function(
    input_head,
    style.tuple_opener(),
    style.delimiter(),
    &[style.tuple_closer(),output_postfix].concat(),
    style)
}
pub(super) fn postfix_function(input_head: &str, arity: usize, output_postfix: &str, style: &RawExpressionStyle) -> ExprPatternPair { allfix_function(
    input_head,
    arity,
    style.tuple_opener(),
    style.delimiter(),
    &[style.tuple_closer(),output_postfix].concat(),
    style)
}

pub(super) fn variadic_outfix_function(input_head: &str, output_left: &str, output_right: &str, style: &RawExpressionStyle) -> ExprPatternPair { variadic_allfix_function(
    input_head,
    output_left,
    style.delimiter(),
    output_right,
    style)
}
pub(super) fn outfix_n_function(input_head: &str, arity: usize, output_left: &str, output_right: &str, style: &RawExpressionStyle) -> ExprPatternPair { allfix_function(
    input_head,
    arity,
    output_left,
    style.delimiter(),
    output_right,
    style)
}

pub(super) fn variadic_allfix_function<'a>(input_head: &str, output_left: &str, output_infix: &str, output_right: &str, raw_style: &RawExpressionStyle) -> ExprPatternPair {
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
pub(super) fn allfix_function<'a>(input_head: &str, arity: usize, output_left: &str, output_infix: &str, output_right: &str, raw_style: &RawExpressionStyle) -> ExprPatternPair {
    if arity < 1 { panic!("Attempted to create n_allfix_function with n < 1") }
    let vars_delimited = (0..arity)
        .map(|x| [
            ExprPatternComponent::new_var(&x.to_string()),
            ExprPatternComponent::new_const(if x == arity { "" } else { raw_style.delimiter() })
        ]).fold(Vec::new(), |x,y| [x.as_slice(),&y].concat());
    let vars_infixed = (0..arity).map(|x| [
            ExprPatternComponent::new_var(&x.to_string()),
            ExprPatternComponent::new_const(if x == arity { "" } else { output_infix })
        ]).fold(Vec::new(), |x,y| [x.as_slice(),&y].concat());
    ExprPatternPair::new(
        ExprPattern::new([
            &[ExprPatternComponent::new_const(raw_style.tuple_opener()),ExprPatternComponent::new_const(input_head)],
            vars_delimited.as_slice(),
            &[ExprPatternComponent::new_const(raw_style.tuple_closer())]
        ].concat()),
        ExprPattern::new([
            &[ExprPatternComponent::new_const(output_left)],
            vars_infixed.as_slice(),
            &[ExprPatternComponent::new_const(output_right)]
        ].concat())
    )
}

#[cfg(test)]
mod tests {
    use std::sync::LazyLock;

    use parsertools::{results::ParseError, Parser};
    use tbl_structures::{atoms::AtomId, expressions::Expression};

    use crate::{helpers::styles::Style, structures::expressions::{functional::special_case::{symbol, symbol_atom, variadic_infix_function, variadic_prefix_function}, patterns::special_case::ExprPatternPair, raw::tests::TEST_RAW_EXPRESSION_STYLE, SpecialCase}};

    fn pre_rtl_test(pair: ExprPatternPair, l: &str, r: &str) -> (Result<String, ParseError<char>>,Result<String, ParseError<char>>) {
        let right = pair.left_to_right().parse_unambiguous(l.chars());
        (right, Ok(r.to_string()))
    }

    fn pre_ltr_test(pair: ExprPatternPair, l: &str, r: &str) -> (Result<String, ParseError<char>>,Result<String, ParseError<char>>) {
        let left = pair.right_to_left().parse_unambiguous(r.chars());
        (left, Ok(l.to_string()))
    }

    #[test]
    fn test_symbol_atom_ltr() {
        let pair = symbol_atom(5, "hello", &TEST_RAW_EXPRESSION_STYLE);
        let (r, r_check) = pre_ltr_test(pair, "#5", "hello");
        assert_eq!(r, r_check)
    }
    #[test]
    fn test_symbol_atom_rtl() {
        let pair = symbol_atom(5, "hello", &TEST_RAW_EXPRESSION_STYLE);
        let (l, l_check) = pre_rtl_test(pair, "#5", "hello");
        assert_eq!(l, l_check)
    }
    #[test]
    fn test_symbol_ltr() {
        let pair = symbol("5", "hello");
        let (r, r_check) = pre_ltr_test(pair, "5", "hello");
        assert_eq!(r, r_check)
    }
    #[test]
    fn test_symbol_rtl() {
        let pair = symbol("5", "hello");
        let (l, l_check) = pre_rtl_test(pair, "5", "hello");
        assert_eq!(l, l_check)
    }


    #[test]
    fn test_variadic_prefix_function_ltr() {
        let pair = variadic_prefix_function("+", "+", &TEST_RAW_EXPRESSION_STYLE);
        let (r, r_check) = pre_ltr_test(pair, "(+,1,2,3,4,5)", "+(1,2,3,4,5)");
        assert_eq!(r, r_check)
    }
    #[test]
    fn test_variadic_prefix_function_rtl() {
        let pair = variadic_prefix_function("+", "+", &TEST_RAW_EXPRESSION_STYLE);
        let (l, l_check) = pre_rtl_test(pair, "(+,1,2,3,4,5)", "+(1,2,3,4,5)");
        assert_eq!(l, l_check)
    }


    #[test]
    fn test_variadic_infix_function_ltr() {
        let pair = variadic_infix_function("+", "+", &TEST_RAW_EXPRESSION_STYLE);
        let (r, r_check) = pre_ltr_test(pair, "(+,1,2,3,4,5)", "(1+2+3+4+5)");
        assert_eq!(r, r_check)
    }
    #[test]
    fn test_variadic_infix_function_rtl() {
        let pair = variadic_infix_function("+", "+", &TEST_RAW_EXPRESSION_STYLE);
        let (l, l_check) = pre_rtl_test(pair, "(+,1,2,3,4,5)", "(1+2+3+4+5)");
        assert_eq!(l, l_check)
    }


    #[test]
    fn test_variadic_postfix_function_ltr() {
        let pair = variadic_prefix_function("+", "+", &TEST_RAW_EXPRESSION_STYLE);
        let (r, r_check) = pre_ltr_test(pair, "(+,1,2,3,4,5)", "+(1,2,3,4,5)");
        assert_eq!(r, r_check)
    }
    #[test]
    fn test_variadic_postfix_function_rtl() {
        let pair = variadic_prefix_function("+", "+", &TEST_RAW_EXPRESSION_STYLE);
        let (l, l_check) = pre_rtl_test(pair, "(+,1,2,3,4,5)", "(1,2,3,4,5)+");
        assert_eq!(l, l_check)
    }
}
