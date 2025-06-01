use std::{sync::LazyLock};

use tbl_structures::{propositions::Expression};

use tbl_stringification::{structures::{expressions::{ExpressionStringifier, NoSpecialCasesStringifier}, vec::VecStringifier}, Stringifier};

use super::atom::NumAtomStringifier;

pub static TERM_STRINGIFIER: LazyLock<Box<dyn Stringifier<Expression>>> = 
    LazyLock::new(|| -> Box<dyn Stringifier<Expression>> { 
        Box::new(ExpressionStringifier::new(
            NumAtomStringifier(),
            VecStringifier::default(),
            NoSpecialCasesStringifier()
        ))
    });

#[cfg(test)]
mod tests {
    use std::{collections::HashMap};

    use tbl_structures::{atoms::AtomId, propositions::Expression};

    use super::*;

    static CONVERSIONS: LazyLock<HashMap<&str,Expression>> = LazyLock::new(|| -> HashMap<&str,Expression> { 
        let atom = |u: usize| { Expression::from(AtomId::try_from(u).unwrap()) };
        HashMap::from_iter(vec![
            ("0",atom(0)),
            ("(0)",Expression::Tuple(vec![atom(0)])),
            ("(0, 1)",Expression::Tuple(vec![atom(0),atom(1)])),
            ("((0), 1)", Expression::from(vec![Expression::from(vec![atom(0)]),atom(1)])),
        ].into_iter())
    });

    #[test]
    fn test_stringify_atom() {
        // Get the string and term to test
        let str = "0";
        let term = CONVERSIONS.get(str).unwrap();
        // Test stringification
        let term_stringified = TERM_STRINGIFIER.stringify(&term);
        assert_eq!(term_stringified,Ok(str.to_string()));
    }
    #[test]
    fn test_destringify_atom() {
        // Get the string and term to test
        let str = "0";
        let term = CONVERSIONS.get(str).unwrap();
        // Test destringification
        let str_destringified = TERM_STRINGIFIER.destringify(&str.to_string());
        assert_eq!(Ok(term.clone()),str_destringified);
    }

    #[test]
    fn test_stringify_unary_tuple() {
        // Get the string and term to test
        let str = "(0)";
        let term = CONVERSIONS.get(str).unwrap();
        // Test stringification
        let term_stringified = TERM_STRINGIFIER.stringify(&term);
        assert_eq!(term_stringified,Ok(str.to_string()));
    }
    #[test]
    fn test_destringify_unary_tuple() {
        // Get the string and term to test
        let str = "(0)";
        let term = CONVERSIONS.get(str).unwrap();
        // Test destringification
        let str_destringified = TERM_STRINGIFIER.destringify(&str.to_string());
        assert_eq!(Ok(term.clone()),str_destringified);
    }

    #[test]
    fn test_stringify_binary_tuple() {
        // Get the string and term to test
        let str = "(0, 1)";
        let term = CONVERSIONS.get(str).unwrap();
        // Test stringification
        let term_stringified = TERM_STRINGIFIER.stringify(&term);
        assert_eq!(term_stringified,Ok(str.to_string()));
    }
    #[test]
    fn test_destringify_binary_tuple() {
        // Get the string and term to test
        let str = "(0, 1)";
        let term = CONVERSIONS.get(str).unwrap();
        // Test destringification
        let str_destringified = TERM_STRINGIFIER.destringify(&str.to_string());
        assert_eq!(Ok(term.clone()),str_destringified);
    }

    #[test]
    fn test_stringify_nested_tuple() {
        // Get the string and term to test
        let str = "((0), 1)";
        let term = CONVERSIONS.get(str).unwrap();
        // Test stringification
        let term_stringified = TERM_STRINGIFIER.stringify(&term);
        assert_eq!(term_stringified,Ok(str.to_string()));
    }
    #[test]
    fn test_destringify_nested_tuple() {
        // Get the string and term to test
        let str = "((0), 1)";
        let term = CONVERSIONS.get(str).unwrap();
        // Test destringification
        let str_destringified = TERM_STRINGIFIER.destringify(&str.to_string());
        assert_eq!(Ok(term.clone()),str_destringified);
    }
}
