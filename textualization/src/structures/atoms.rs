use parsertools::parsers::Parser;
use tbl_structures::atoms::AtomId;

use crate::helpers::parsers::{num_parser, string_parser};


#[derive(Clone)]
pub struct AtomStyle { atom_id_indicator: String }
impl AtomStyle {
    pub fn new(atom_id_indicator: String) -> Result<Self,()>
        { Ok(Self { atom_id_indicator }) }
    pub fn from_strs(atom_id_indicator: &str) -> Self
        { Self { atom_id_indicator: atom_id_indicator.to_string(), } }

    pub fn id_indicator(&self) -> &String { &self.atom_id_indicator }

    pub fn to_id(&self, num: usize) -> String { self.atom_id_indicator.clone() + &num.to_string() }
}
impl Default for AtomStyle {
    fn default() -> Self {
        Self { atom_id_indicator: "#".to_string() }
    }
}

pub fn atom_id_parser<'a>(style: &AtomStyle) -> Parser<'a, char,AtomId> {
    string_parser(style.id_indicator()).unwrap().then(
        num_parser()
    ).map(|(_, uint)| -> AtomId { AtomId::try_from(uint).unwrap() })
}

#[cfg(test)]
pub (crate) mod tests {
    use std::sync::LazyLock;

    use crate::test_helpers::parse_str;

    use super::*;

    pub (crate) const TEST_ATOM_STYLE: LazyLock<AtomStyle> = LazyLock::new(|| -> AtomStyle {
        AtomStyle::from_strs("#")
    });
    
    #[test]
    fn test_id_parser_with_atom_id() {
        assert_eq!(parse_str(atom_id_parser(&TEST_ATOM_STYLE), "#1124"),Ok(AtomId(1124)))
    }
    #[test]
    fn test_id_parser_with_plain_num() {
        assert!(parse_str(atom_id_parser(&TEST_ATOM_STYLE), "1124").is_err())
    }
    #[test]
    fn test_id_parser_with_atom_symbol() {
        assert!(parse_str(atom_id_parser(&TEST_ATOM_STYLE), "P").is_err())
    }
}
