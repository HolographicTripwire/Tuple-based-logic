use std::sync::LazyLock;

use tbl_textualization::structures::atom::AtomTextualizer;

pub static ATOM_TEXTUALIZER: LazyLock<AtomTextualizer> = LazyLock::new(|| -> AtomTextualizer { 
    AtomTextualizer::from_strings(vec![
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
    ])
});
