use std::sync::LazyLock;

use crate::structures::atom::{construct_symbols, AtomStringifier};

pub static STRINGIFIER: LazyLock<AtomStringifier> = LazyLock::new(|| -> AtomStringifier { construct_symbols(vec![
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
])});
