use tbl_structures::propositions::Expression;

use std::{sync::LazyLock};

use tbl_textualization::{structures::{expressions::{special_cases::SpecialCaseTextualizerSet, ExpressionTextualizer}, vec::VecTextualizer}, Textualizer};

pub static TERM_TEXTUALIZER: LazyLock<Box<dyn Textualizer<Expression>>> = 
    LazyLock::new(|| -> Box<dyn Textualizer<Expression>> { 
        Box::new(ExpressionTextualizer::new(
            super::atom::ATOM_TEXTUALIZER.clone(),
            VecTextualizer::default(),
            SpecialCaseTextualizerSet::default()
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
        let term_stringified = TERM_TEXTUALIZER.textualize(&term);
        assert_eq!(term_stringified,Ok(str.to_string()));
    }
    #[test]
    fn test_destringify_atom() {
        // Get the string and term to test
        let str = "∧";
        let term = CONVERSIONS.get(str).unwrap();
        // Test destringification
        let str_destringified = TERM_TEXTUALIZER.detextualize(&str.to_string());
        assert_eq!(Ok(term.clone()),str_destringified);
    }

    #[test]
    fn test_stringify_unary_tuple() {
        // Get the string and term to test
        let str = "(∧)";
        let term = CONVERSIONS.get(str).unwrap();
        // Test stringification
        let term_stringified = TERM_TEXTUALIZER.textualize(&term);
        assert_eq!(term_stringified,Ok(str.to_string()));
    }
    #[test]
    fn test_destringify_unary_tuple() {
        // Get the string and term to test
        let str = "(∧)";
        let term = CONVERSIONS.get(str).unwrap();
        // Test destringification
        let str_destringified = TERM_TEXTUALIZER.detextualize(&str.to_string());
        assert_eq!(Ok(term.clone()),str_destringified);
    }

    #[test]
    fn test_stringify_binary_tuple() {
        // Get the string and term to test
        let str = "(∧, →)";
        let term = CONVERSIONS.get(str).unwrap();
        // Test stringification
        let term_stringified = TERM_TEXTUALIZER.textualize(&term);
        assert_eq!(term_stringified,Ok(str.to_string()));
    }
    #[test]
    fn test_destringify_binary_tuple() {
        // Get the string and term to test
        let str = "(∧, →)";
        let term = CONVERSIONS.get(str).unwrap();
        // Test destringification
        let str_destringified = TERM_TEXTUALIZER.detextualize(&str.to_string());
        assert_eq!(Ok(term.clone()),str_destringified);
    }

    #[test]
    fn test_stringify_nested_tuple() {
        // Get the string and term to test
        let str = "((∧), →)";
        let term = CONVERSIONS.get(str).unwrap();
        // Test stringification
        let term_stringified = TERM_TEXTUALIZER.textualize(&term);
        assert_eq!(term_stringified,Ok(str.to_string()));
    }
    #[test]
    fn test_destringify_nested_tuple() {
        // Get the string and term to test
        let str = "((∧), →)";
        let term = CONVERSIONS.get(str).unwrap();
        // Test destringification
        let str_destringified = TERM_TEXTUALIZER.detextualize(&str.to_string());
        assert_eq!(Ok(term.clone()),str_destringified);
    }
}
