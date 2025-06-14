
use parsertools::parsers::{helpers::lazy, Parser};
use tbl_structures::propositions::Expression;

use crate::{helpers::string_parser,structures::atoms::{atom_parser, AtomControls}};

#[derive(Clone)]
pub struct RawExpressionControls {
    atom_controls: AtomControls,
    tuple_opener: String,
    tuple_closer: String,
    delimiter: String
}
impl RawExpressionControls {
    pub fn new(atoms: AtomControls, tuple_opener: String, tuple_closer: String, delimiter: String) -> Self { Self { atom_controls: atoms, tuple_opener, tuple_closer, delimiter } }
    pub fn from_strs(atom_controls: AtomControls, tuple_opener: &str, tuple_closer: &str, delimiter: &str) -> Self {
        Self { atom_controls, tuple_opener: tuple_opener.to_string(), tuple_closer: tuple_closer.to_string(), delimiter: delimiter.to_string() }
    }

    pub fn atom_controls(&self) -> &AtomControls { &self.atom_controls }
    pub fn tuple_opener(&self) -> &String { &self.tuple_opener }
    pub fn tuple_closer(&self) -> &String { &self.tuple_closer }
    pub fn delimiter(&self) -> &String { &self.delimiter }
}

pub fn raw_expression_parser<'a>(controls: &RawExpressionControls) -> Parser<'a, char,Expression> {
    let opener = string_parser(controls.tuple_opener()).unwrap();
    let closer = string_parser(controls.tuple_closer()).unwrap();
    let atom = atomic_expression_parser(controls);
    let empty_tuple = opener.clone().then(closer.clone()).map(|_| Expression::Tuple(vec![]));
    let filled_tuple = opener.then(raw_expression_series_parser(controls.clone())).then(closer)
        .map(|((_,expr),_)| expr);
    atom.or(empty_tuple).or(filled_tuple)
}

fn raw_expression_series_parser<'a>(controls: RawExpressionControls) -> Parser<'a,char,Expression> {
    let delimiter = string_parser(controls.delimiter()).unwrap();
    let binding = controls.clone();
    
    let unary = lazy(move || raw_expression_parser(&binding.clone())
        .map(|expr| Expression::Tuple(vec![expr])));
    
    let series = lazy(move || raw_expression_series_parser(controls.clone()));
    let non_unary = unary.clone().then(delimiter).then(series)
        .map(|((a,_),b)| Expression::Tuple([a.as_slice().unwrap(),b.as_slice().unwrap()].concat()) );
    
    non_unary.or(unary)
}

fn atomic_expression_parser<'a>(controls: &RawExpressionControls) -> Parser<'a, char, Expression> {
    atom_parser(controls.atom_controls()).map(|atom| Expression::from(atom))
}

#[cfg(test)]
pub (crate) mod tests {
    use std::sync::LazyLock;

    use tbl_structures::atoms::AtomId;

    use crate::{test_helpers::parse_str,structures::atoms::tests::TEST_ATOM_CONTROLS};

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

    #[test]
    fn test_expression_parser_with_atom_id() {
        let expected = Ok(Expression::Atomic(AtomId(1124).into()));
        assert_eq!(parse_str(raw_expression_parser(&TEST_RAW_EXPRESSION_CONTROLS), "#1124"), expected)
    }
    #[test]
    fn test_expression_parser_with_plain_num() {
        assert!(parse_str(raw_expression_parser(&TEST_RAW_EXPRESSION_CONTROLS), "1124").is_err())
    }
    #[test]
    fn test_expression_parser_with_atom_symbol() {
        let expected = Ok(Expression::Atomic(AtomId(8).into()));
        assert_eq!(parse_str(raw_expression_parser(&TEST_RAW_EXPRESSION_CONTROLS), "P"), expected)
    }
    #[test]
    fn test_expression_parser_with_atom_series() {
        assert!(parse_str(raw_expression_parser(&TEST_RAW_EXPRESSION_CONTROLS), "P,Q").is_err())
    }
    #[test]
    fn test_expression_parser_with_atom_id_tuple() {
        let expected = Ok(Expression::Tuple(vec![AtomId(8).into(),AtomId(241).into()]));
        assert_eq!(parse_str(raw_expression_parser(&TEST_RAW_EXPRESSION_CONTROLS), "(P,#241)"), expected)
    }
    #[test]
    fn test_expression_parser_with_nested_tuple() {
        let neg_p = Expression::Tuple(vec![AtomId(3).into(),AtomId(8).into()]);
        let neg_neg_p = Expression::Tuple(vec![AtomId(3).into(),neg_p]);
        let expected = Ok(Expression::Tuple(vec![AtomId(4).into(),AtomId(8).into(),neg_neg_p]));
        assert_eq!(parse_str(raw_expression_parser(&TEST_RAW_EXPRESSION_CONTROLS), "(=,P,(¬,(¬,P)))"), expected)
    }
    #[test]
    fn test_expression_parser_with_empty_tuple() {
        let empty = Expression::Tuple(vec![]);
        let expected = Ok(Expression::Tuple(vec![AtomId(4).into(),AtomId(13).into(),empty]));
        assert_eq!(parse_str(raw_expression_parser(&TEST_RAW_EXPRESSION_CONTROLS), "(=,#13,())"), expected)
    }
}
