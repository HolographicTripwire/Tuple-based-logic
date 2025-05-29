use tbl_stringification::{terms::{NoSpecialCasesStringifier, TermStringifier}, Stringifier};
use tbl_structures::propositions::Term;

use std::{sync::LazyLock};

use crate::stringifiers::{formats::plaintext,vec::VecStringifier};

pub static TERM_STRINGIFIER: LazyLock<Box<dyn Stringifier<Term>>> = 
    LazyLock::new(|| -> Box<dyn Stringifier<Term>> { 
        Box::new(TermStringifier::new(
            plaintext::atom::STRINGIFIER.clone(),
            VecStringifier::default(),
            NoSpecialCasesStringifier()
        ))
    });

#[cfg(test)]
mod tests {
    use std::{collections::HashMap};

    use tbl_structures::{atoms::BuiltInAtom, propositions::Term};

    use super::*;

    static CONVERSIONS: LazyLock<HashMap<&str,Term>> = LazyLock::new(|| -> HashMap<&str,Term> { 
        let conjunction = || Term::from(BuiltInAtom::Conjunction);
        let implication = || Term::from(BuiltInAtom::Implication);
        HashMap::from_iter(vec![
            ("∧",conjunction()),
            ("(∧)",Term::Tuple(vec![conjunction()])),
            ("(∧, →)",Term::Tuple(vec![conjunction(),implication()])),
            ("((∧), →)", Term::from(vec![Term::from(vec![conjunction()]),implication()])),
        ].into_iter())
    });

    #[test]
    fn test_stringify_atom() {
        // Get the string and term to test
        let str = "∧";
        let term = CONVERSIONS.get(str).unwrap();
        // Test stringification
        let term_stringified = TERM_STRINGIFIER.stringify(&term);
        assert_eq!(term_stringified,Ok(str.to_string()));
    }
    #[test]
    fn test_destringify_atom() {
        // Get the string and term to test
        let str = "∧";
        let term = CONVERSIONS.get(str).unwrap();
        // Test destringification
        let str_destringified = TERM_STRINGIFIER.destringify(&str.to_string());
        assert_eq!(Ok(term.clone()),str_destringified);
    }

    #[test]
    fn test_stringify_unary_tuple() {
        // Get the string and term to test
        let str = "(∧)";
        let term = CONVERSIONS.get(str).unwrap();
        // Test stringification
        let term_stringified = TERM_STRINGIFIER.stringify(&term);
        assert_eq!(term_stringified,Ok(str.to_string()));
    }
    #[test]
    fn test_destringify_unary_tuple() {
        // Get the string and term to test
        let str = "(∧)";
        let term = CONVERSIONS.get(str).unwrap();
        // Test destringification
        let str_destringified = TERM_STRINGIFIER.destringify(&str.to_string());
        assert_eq!(Ok(term.clone()),str_destringified);
    }

    #[test]
    fn test_stringify_binary_tuple() {
        // Get the string and term to test
        let str = "(∧, →)";
        let term = CONVERSIONS.get(str).unwrap();
        // Test stringification
        let term_stringified = TERM_STRINGIFIER.stringify(&term);
        assert_eq!(term_stringified,Ok(str.to_string()));
    }
    #[test]
    fn test_destringify_binary_tuple() {
        // Get the string and term to test
        let str = "(∧, →)";
        let term = CONVERSIONS.get(str).unwrap();
        // Test destringification
        let str_destringified = TERM_STRINGIFIER.destringify(&str.to_string());
        assert_eq!(Ok(term.clone()),str_destringified);
    }

    #[test]
    fn test_stringify_nested_tuple() {
        // Get the string and term to test
        let str = "((∧), →)";
        let term = CONVERSIONS.get(str).unwrap();
        // Test stringification
        let term_stringified = TERM_STRINGIFIER.stringify(&term);
        assert_eq!(term_stringified,Ok(str.to_string()));
    }
    #[test]
    fn test_destringify_nested_tuple() {
        // Get the string and term to test
        let str = "((∧), →)";
        let term = CONVERSIONS.get(str).unwrap();
        // Test destringification
        let str_destringified = TERM_STRINGIFIER.destringify(&str.to_string());
        assert_eq!(Ok(term.clone()),str_destringified);
    }
}
