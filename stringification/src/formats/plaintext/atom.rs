use std::sync::LazyLock;

use crate::structures::atom::{construct_symbols, AtomStringifier};

pub static STRINGIFIER: LazyLock<AtomStringifier> = LazyLock::new(|| -> AtomStringifier { construct_symbols(vec![
    // Built-in atoms
    (000,"∧"),  // Conjunction
    (001,"∀"),  // Universal quantiifer
    (002,"→"),  // Implication
    (004,"="),  // Identity
    (003,"¬"),  // Negation
    (006,"⟨⟩"), // Verbatim
    (007,"⌢"),  // Concatenation
    (008,"⚛"),  // Atomicity
    // Not built-in
])});
