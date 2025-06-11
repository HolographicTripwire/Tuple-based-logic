
use parsertools::{dynamic, Parser};
use tbl_structures::propositions::Expression;

use crate::{atom::{atom_parser, AtomControls}, helpers::string_parser};

#[derive(Clone)]
pub struct RawExpressionControls {
    atoms: AtomControls,
    tuple_opener: String,
    tuple_closer: String,
    delimiter: String
}
impl RawExpressionControls {
    pub fn new(atoms: AtomControls, tuple_opener: String, tuple_closer: String, delimiter: String) -> Self { Self { atoms, tuple_opener, tuple_closer, delimiter } }
    pub fn from_strs(atoms: AtomControls, tuple_opener: &str, tuple_closer: &str, delimiter: &str) -> Self {
        Self { atoms, tuple_opener: tuple_opener.to_string(), tuple_closer: tuple_closer.to_string(), delimiter: delimiter.to_string() }
    }

    pub fn atoms(&self) -> &AtomControls { &self.atoms }
    pub fn tuple_opener(&self) -> &String { &self.tuple_opener }
    pub fn tuple_closer(&self) -> &String { &self.tuple_closer }
    pub fn delimiter(&self) -> &String { &self.delimiter }
}

fn atomic_expression_parser<'a>(controls: &RawExpressionControls) -> Parser<'a, char, Expression> {
    atom_parser(controls.atoms()).map(|atom| Expression::from(atom))
}

#[cfg(test)]
pub (crate) mod tests {
    use std::sync::LazyLock;

    use tbl_structures::atoms::AtomId;

    use crate::{test_helpers::parse_str,atom::tests::TEST_ATOM_CONTROLS};

    use super::*;
    
    pub (crate) const TEST_RAW_EXPRESSION_CONTROLS: LazyLock<RawExpressionControls> = LazyLock::new(|| -> RawExpressionControls {
        RawExpressionControls::from_strs(TEST_ATOM_CONTROLS.clone(), "(", ")", ",")
    });

    #[test]
    fn test_atomic_parser_with_atom_id() {
        assert_eq!(parse_str(atomic_expression_parser(&TEST_RAW_EXPRESSION_CONTROLS), "#1124"),Ok(Expression::Atomic(AtomId(1124))))
    }
    #[test]
    fn test_atomic_parser_with_plain_num() {
        assert!(parse_str(atomic_expression_parser(&TEST_RAW_EXPRESSION_CONTROLS), "1124").is_err())
    }
    #[test]
    fn test_atomic_parser_with_atom_symbol() {
        assert_eq!(parse_str(atomic_expression_parser(&TEST_RAW_EXPRESSION_CONTROLS), "P"), Ok(Expression::Atomic(AtomId(8))))
    }
    #[test]
    fn test_atomic_parser_with_atom_series() {
        assert!(parse_str(atomic_expression_parser(&TEST_RAW_EXPRESSION_CONTROLS), "P,Q").is_err())
    }
    #[test]
    fn test_atomic_parser_with_atom_tuple() {
        assert!(parse_str(atomic_expression_parser(&TEST_RAW_EXPRESSION_CONTROLS), "(P,Q)").is_err())
    }


    #[test]
    fn test_series_parser_with_atom_id() {
        let expected = Ok(Expression::Tuple(vec![AtomId(1124).into()]));
        assert_eq!(parse_str(raw_expression_series_parser(TEST_RAW_EXPRESSION_CONTROLS.clone()), "#1124"), expected)
    }
    #[test]
    fn test_series_parser_with_plain_num() {
        assert!(parse_str(raw_expression_series_parser(TEST_RAW_EXPRESSION_CONTROLS.clone()), "1124").is_err())
    }
    #[test]
    fn test_series_parser_with_atom_symbol() {
        let expected = Ok(Expression::Tuple(vec![AtomId(8).into()]));
        assert_eq!(parse_str(raw_expression_series_parser(TEST_RAW_EXPRESSION_CONTROLS.clone()), "P"), expected)
    }
    #[test]
    fn test_series_parser_with_atom_series() {
        let expected = Ok(Expression::Tuple(vec![AtomId(8).into(),AtomId(9).into()]));
        assert_eq!(parse_str(raw_expression_series_parser(TEST_RAW_EXPRESSION_CONTROLS.clone()), "P,Q"), expected)
    }
    #[test]
    fn test_series_parser_with_atom_series_wrong_delimiter() {
        assert!(parse_str(raw_expression_series_parser(TEST_RAW_EXPRESSION_CONTROLS.clone()), "P Q").is_err())
    }
    #[test]
    fn test_series_parser_with_atom_tuple() {
        let expected = Ok(Expression::Tuple(vec![Expression::Tuple(vec![AtomId(8).into(),AtomId(9).into()])]));
        assert_eq!(parse_str(raw_expression_series_parser(TEST_RAW_EXPRESSION_CONTROLS.clone()), "(P,Q)"), expected)
    }
}
