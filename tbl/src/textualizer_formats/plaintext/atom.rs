use std::sync::LazyLock;

use tbl_structures::atoms::AtomId;
use tbl_textualization::{structures::atom::AtomParser, Stringifier};

pub static ATOM_PARSER: LazyLock<Box<dyn Stringifier<AtomId>>> = LazyLock::new(|| -> Box<dyn Stringifier<AtomId>> { 
    Box::new(AtomParser::from_strings(vec![
        // Built-in atoms
        (000,"∧"),  // Conjunction
        (001,"∀"),  // Universal quantiifer
        (002,"→"),  // Implication
        (003,"¬"),  // Negation
        (004,"="),  // Identity
        (005,"⟨⟩"), // Verbatim
        (006,"⌢"),  // Concatenation
        (007,"⚛"),  // Atomicity
        // Not built-in
    ]))
});
