use std::sync::LazyLock;

use tbl_stringification::structures::atom::AtomStringifier;

pub static STRINGIFIER: LazyLock<AtomStringifier> = LazyLock::new(|| -> AtomStringifier { 
    AtomStringifier::from_strings(vec![
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
