use std::sync::{Arc, LazyLock};

use tbl_textualization::{structures::{atoms::AtomStyle, expressions::{functional::SpecialCasesBuilder, raw::RawExpressionStyle, ExpressionStyle}}};

pub const EXPRESSION_STYLE: LazyLock<ExpressionStyle> = LazyLock::new(|| -> ExpressionStyle {
    let atom_style = AtomStyle::from_strs("#");
    let raw_expression_style  = RawExpressionStyle::from_strs(atom_style, "(", ")", ",");
    let special_cases = SpecialCasesBuilder::new(raw_expression_style.clone()).build();
    ExpressionStyle::new(raw_expression_style, Arc::new(special_cases))
});

#[cfg(test)]
mod tests {
    use std::{collections::HashMap};

    use tbl_structures::{atoms::AtomId, propositions::Expression};
    use tbl_textualization::{helpers::styles::Stylable, structures::expressions::expression_parser};

    use super::*;

    static CONVERSIONS: LazyLock<HashMap<&str,Expression>> = LazyLock::new(|| -> HashMap<&str,Expression> { 
        let atom = |u: usize| { Expression::from(AtomId::try_from(u).unwrap()) };
        HashMap::from_iter(vec![
            ("#0",atom(0)),
            ("(#0)",Expression::Tuple(vec![atom(0)])),
            ("(#0, #1)",Expression::Tuple(vec![atom(0),atom(1)])),
            ("((#0), #1)", Expression::from(vec![Expression::from(vec![atom(0)]),atom(1)])),
        ].into_iter())
    });

    #[test]
    fn test_stringify_atom() {
        // Get the string and expression to test
        let str = "#0";
        let expression = CONVERSIONS.get(str).unwrap();
        // Test stringification
        let expression_stringified = expression.styled(&EXPRESSION_STYLE.clone()).to_string();
        assert_eq!(expression_stringified,str.to_string());
    }
    #[test]
    fn test_destringify_atom() {
        // Get the string and expression to test
        let str = "#0";
        let expression = CONVERSIONS.get(str).unwrap();
        // Test destringification
        let parser = expression_parser(EXPRESSION_STYLE.clone());
        let str_as_expression = parser.parse(str.chars());
        assert_eq!(Ok(expression.clone()),str_as_expression);
    }

    #[test]
    fn test_stringify_unary_tuple() {
        // Get the string and expression to test
        let str = "(#0)";
        let expression = CONVERSIONS.get(str).unwrap();
        // Test stringification
        let expression_stringified = expression.styled(&EXPRESSION_STYLE.clone()).to_string();
        assert_eq!(expression_stringified,str.to_string());
    }
    #[test]
    fn test_destringify_unary_tuple() {
        // Get the string and expression to test
        let str = "(#0)";
        let expression = CONVERSIONS.get(str).unwrap();
        // Test destringification
        let parser = expression_parser(EXPRESSION_STYLE.clone());
        let str_as_expression = parser.parse(str.chars());
        assert_eq!(Ok(expression.clone()),str_as_expression);
    }

    #[test]
    fn test_stringify_binary_tuple() {
        // Get the string and expression to test
        let str = "(#0, #1)";
        let expression = CONVERSIONS.get(str).unwrap();
        // Test stringification
        let expression_stringified = expression.styled(&EXPRESSION_STYLE.clone()).to_string();
        assert_eq!(expression_stringified,str.to_string());
    }
    #[test]
    fn test_destringify_binary_tuple() {
        // Get the string and expression to test
        let str = "(#0, #1)";
        let expression = CONVERSIONS.get(str).unwrap();
        // Test destringification
        let parser = expression_parser(EXPRESSION_STYLE.clone());
        let str_as_expression = parser.parse(str.chars());
        assert_eq!(Ok(expression.clone()),str_as_expression);
    }

    #[test]
    fn test_stringify_nested_tuple() {
        // Get the string and expression to test
        let str = "((#0), #1)";
        let expression = CONVERSIONS.get(str).unwrap();
        // Test stringification
        let expression_stringified = expression.styled(&EXPRESSION_STYLE.clone()).to_string();
        assert_eq!(expression_stringified,str.to_string());
    }
    #[test]
    fn test_destringify_nested_tuple() {
        // Get the string and expression to test
        let str = "((#0), #1)";
        let expression = CONVERSIONS.get(str).unwrap();
        // Test destringification
        let parser = expression_parser(EXPRESSION_STYLE.clone());
        let str_as_expression = parser.parse(str.chars());
        assert_eq!(Ok(expression.clone()),str_as_expression);
    }
}
