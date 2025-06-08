use std::sync::LazyLock;

use tbl_structures::atoms::AtomId;
use tbl_textualization::{structures::atom::AtomParser, Stringifier};

pub static ATOM_PARSER: LazyLock<Box<dyn Stringifier<AtomId>>> = LazyLock::new(|| -> Box<dyn Stringifier<AtomId>> { 
    Box::new(AtomParser::from_strings(vec![
        // No built in atoms interpretations; they will all be treated as numbers
    ]))
});
