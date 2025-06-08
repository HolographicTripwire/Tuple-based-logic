use tbl_structures::propositions::Expression;
use tbl_textualization::{structures::{expressions::{patterns::lexer::ExprPatternLexer, ExpressionParser}, vec::VecParser}, Stringifier};

use std::{sync::LazyLock};

pub static VEC_PARSER: LazyLock<Box<dyn Stringifier<Vec<String>>>> =
    LazyLock::new(|| -> Box<dyn Stringifier<Vec<String>>> {
        Box::new(VecParser::default())
    });
pub static EXPR_PATTERN_LEXER: LazyLock<Box<ExprPatternLexer>> =
    LazyLock::new(|| -> Box<ExprPatternLexer> {
        Box::new(ExprPatternLexer::default())
    });
pub static EXPR_PARSER: LazyLock<Box<dyn Stringifier<Expression>>> = 
    LazyLock::new(|| -> Box<dyn Stringifier<Expression>> { 
        Box::new(ExpressionParser::new(
            super::atom::ATOM_PARSER.clone(),
            VEC_PARSER.clone(),
            super::special_cases::SPECIAL_CASE_PARSER.clone()
        ))
    });

#[cfg(test)]
mod tests {
    use std::{collections::HashMap};

    use tbl_structures::{atoms::BuiltInAtom, propositions::Expression};

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
        // Get the string and term to test
        let str = "∧";
        let term = CONVERSIONS.get(str).unwrap();
        // Test stringification
        let term_stringified = EXPR_PARSER.stringify(&term);
        assert_eq!(term_stringified,Ok(str.to_string()));
    }
    #[test]
    fn test_destringify_atom() {
        // Get the string and term to test
        let str = "∧";
        let term = CONVERSIONS.get(str).unwrap();
        // Test destringification
        let str_destringified = EXPR_PARSER.destringify(&str.to_string());
        assert_eq!(Ok(term.clone()),str_destringified);
    }

    #[test]
    fn test_stringify_unary_tuple() {
        // Get the string and term to test
        let str = "(∧)";
        let term = CONVERSIONS.get(str).unwrap();
        // Test stringification
        let term_stringified = EXPR_PARSER.stringify(&term);
        assert_eq!(term_stringified,Ok(str.to_string()));
    }
    #[test]
    fn test_destringify_unary_tuple() {
        // Get the string and term to test
        let str = "(∧)";
        let term = CONVERSIONS.get(str).unwrap();
        // Test destringification
        let str_destringified = EXPR_PARSER.destringify(&str.to_string());
        assert_eq!(Ok(term.clone()),str_destringified);
    }

    #[test]
    fn test_stringify_binary_tuple() {
        // Get the string and term to test
        let str = "(∧, →)";
        let term = CONVERSIONS.get(str).unwrap();
        // Test stringification
        let term_stringified = EXPR_PARSER.stringify(&term);
        assert_eq!(term_stringified,Ok(str.to_string()));
    }
    #[test]
    fn test_destringify_binary_tuple() {
        // Get the string and term to test
        let str = "(∧, →)";
        let term = CONVERSIONS.get(str).unwrap();
        // Test destringification
        let str_destringified = EXPR_PARSER.destringify(&str.to_string());
        assert_eq!(Ok(term.clone()),str_destringified);
    }

    #[test]
    fn test_stringify_nested_tuple() {
        // Get the string and term to test
        let str = "((∧), →)";
        let term = CONVERSIONS.get(str).unwrap();
        // Test stringification
        let term_stringified = EXPR_PARSER.stringify(&term);
        assert_eq!(term_stringified,Ok(str.to_string()));
    }
    #[test]
    fn test_destringify_nested_tuple() {
        // Get the string and term to test
        let str = "((∧), →)";
        let term = CONVERSIONS.get(str).unwrap();
        // Test destringification
        let str_destringified = EXPR_PARSER.destringify(&str.to_string());
        assert_eq!(Ok(term.clone()),str_destringified);
    }
}
