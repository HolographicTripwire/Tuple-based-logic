use parsertools::parsers::Parser;
use tbl_structures::atoms::AtomId;

use crate::helpers::{num_parser, string_parser};


#[derive(Clone)]
pub struct AtomControls { atom_id_indicator: String }
impl AtomControls {
    pub fn new(atom_id_indicator: String) -> Result<Self,()>
        { Ok(Self { atom_id_indicator }) }
    pub fn from_strs(atom_id_indicator: &str) -> Self
        { Self { atom_id_indicator: atom_id_indicator.to_string(), } }

    pub fn id_indicator(&self) -> &String { &self.atom_id_indicator }

    pub fn to_id(&self, num: usize) -> String { self.atom_id_indicator.clone() + &num.to_string() }
}
impl Default for AtomControls {
    fn default() -> Self {
        Self { atom_id_indicator: "#".to_string() }
    }
}

pub fn atom_id_parser<'a>(controls: &AtomControls) -> Parser<'a, char,AtomId> {
    string_parser(controls.id_indicator()).unwrap().then(
        num_parser()
    ).map(|(_, uint)| -> AtomId { AtomId::try_from(uint).unwrap() })
}

#[cfg(test)]
pub (crate) mod tests {
    use std::sync::LazyLock;

    use crate::test_helpers::parse_str;

    use super::*;

    pub (crate) const TEST_ATOM_CONTROLS: LazyLock<AtomControls> = LazyLock::new(|| -> AtomControls {
        AtomControls::from_strs("#")
    });
    
    #[test]
    fn test_id_parser_with_atom_id() {
        assert_eq!(parse_str(atom_id_parser(&TEST_ATOM_CONTROLS), "#1124"),Ok(AtomId(1124)))
    }
    #[test]
    fn test_id_parser_with_plain_num() {
        assert!(parse_str(atom_id_parser(&TEST_ATOM_CONTROLS), "1124").is_err())
    }
    #[test]
    fn test_id_parser_with_atom_symbol() {
        assert!(parse_str(atom_id_parser(&TEST_ATOM_CONTROLS), "P").is_err())
    }
}
