use std::sync::{Arc, LazyLock};

use tbl_textualization::structures::{atoms::AtomStyle, expressions::{functional::SpecialCasesBuilder, raw::RawExpressionStyle, ExpressionStyle, SpecialCases}};

const RAW_EXPRESSION_STYLE: LazyLock<RawExpressionStyle> = LazyLock::new(|| {
    let atom_style = AtomStyle::from_strs("#");
    RawExpressionStyle::from_strs(atom_style, "(", ")", ", ")
});

const SPECIAL_CASES: LazyLock<SpecialCases> = LazyLock::new(|| SpecialCasesBuilder::new(RAW_EXPRESSION_STYLE.clone())
    // Built in atoms
    .add_variadic_atomic_infix_function(000,"∧", " ∧ ")  // Conjunction
    .add_atomic_prefix_function(001, 1..1,"∀","∀")  // Universal quantiifer
    .add_atomic_infix_function(002, 2..2, "→"," → ")  // Implication
    .add_atomic_prefix_function(003,1..1, "¬","¬")  // Negation
    .add_variadic_atomic_infix_function(004,"=", " = ")  // Identity
    .add_variadic_atomic_outfix_function(005,"⟨⟩","⟨","⟩") // Verbatim
    .add_variadic_atomic_infix_function(006,"⌢","⌢")  // Concatenation
    .add_atomic_prefix_function(007,1..1, "⚛","⚛")  // Atomicity
    // Non-built-in atoms
    .build()
);

pub const EXPRESSION_STYLE: LazyLock<ExpressionStyle> = LazyLock::new(|| -> ExpressionStyle {
    ExpressionStyle::new(RAW_EXPRESSION_STYLE.clone(), Arc::new(SPECIAL_CASES.clone()))
});

#[cfg(test)]
mod tests {
    use std::{collections::HashMap};

    use tbl_structures::{atoms::BuiltInAtom, propositions::Expression};
    use tbl_textualization::{helpers::styles::Stylable, structures::expressions::expression_parser};

    use super::*;

    static CONVERSIONS: LazyLock<HashMap<&str,Expression>> = LazyLock::new(|| -> HashMap<&str,Expression> { 
        let conjunction = || Expression::from(BuiltInAtom::Conjunction);
        let implication = || Expression::from(BuiltInAtom::Implication);
        HashMap::from_iter(vec![
            ("∧",conjunction()),
            ("(∧)",Expression::Tuple(vec![conjunction()])),
            ("(∧, →)",Expression::Tuple(vec![conjunction(),implication()])),
            ("((∧), →)", Expression::from(vec![Expression::from(vec![conjunction()]),implication()])),
        ].into_iter())
    });

    #[test]
    fn test_stringify_atom() {
        // Get the string and expression to test
        let str = "∧";
        let expression = CONVERSIONS.get(str).unwrap();
        // Test stringification
        let expression_stringified = expression.styled(&EXPRESSION_STYLE.clone()).to_string();
        assert_eq!(expression_stringified,str.to_string());
    }
    #[test]
    fn test_destringify_atom() {
        // Get the string and expression to test
        let str = "∧";
        let expression = CONVERSIONS.get(str).unwrap();
        // Test destringification
        let parser = expression_parser(EXPRESSION_STYLE.clone());
        let str_as_expression = parser.parse_unambiguous(str.chars());
        assert_eq!(Ok(expression.clone()),str_as_expression);
    }

    #[test]
    fn test_stringify_unary_tuple() {
        // Get the string and expression to test
        let str = "(∧)";
        let expression = CONVERSIONS.get(str).unwrap();
        // Test stringification
        let expression_stringified = expression.styled(&EXPRESSION_STYLE.clone()).to_string();
        assert_eq!(expression_stringified,str.to_string());
    }
    #[test]
    fn test_destringify_unary_tuple() {
        // Get the string and expression to test
        let str = "(∧)";
        let expression = CONVERSIONS.get(str).unwrap();
        // Test destringification
        let parser = expression_parser(EXPRESSION_STYLE.clone());
        let str_as_expression = parser.parse_unambiguous(str.chars());
        assert_eq!(Ok(expression.clone()),str_as_expression);
    }

    #[test]
    fn test_stringify_binary_tuple() {
        // Get the string and expression to test
        let str = "(∧, →)";
        let expression = CONVERSIONS.get(str).unwrap();
        // Test stringification
        let expression_stringified = expression.styled(&EXPRESSION_STYLE.clone()).to_string();
        assert_eq!(expression_stringified,str.to_string());
    }
    #[test]
    fn test_destringify_binary_tuple() {
        // Get the string and expression to test
        let str = "(∧, →)";
        let expression = CONVERSIONS.get(str).unwrap();
        // Test destringification
        let parser = expression_parser(EXPRESSION_STYLE.clone());
        let str_as_expression = parser.parse_unambiguous(str.chars());
        assert_eq!(Ok(expression.clone()),str_as_expression);
    }

    #[test]
    fn test_stringify_nested_tuple() {
        // Get the string and expression to test
        let str = "((∧), →)";
        let expression = CONVERSIONS.get(str).unwrap();
        // Test stringification
        let expression_stringified = expression.styled(&EXPRESSION_STYLE.clone()).to_string();
        assert_eq!(expression_stringified,str.to_string());
    }
    #[test]
    fn test_destringify_nested_tuple() {
        // Get the string and expression to test
        let str = "((∧), →)";
        let expression = CONVERSIONS.get(str).unwrap();
        // Test destringification
        let parser = expression_parser(EXPRESSION_STYLE.clone());
        let str_as_expression = parser.parse_unambiguous(str.chars());
        assert_eq!(Ok(expression.clone()),str_as_expression);
    }
}
