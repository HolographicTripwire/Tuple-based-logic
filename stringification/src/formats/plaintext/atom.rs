use std::sync::LazyLock;

use crate::structures::atom::{construct_symbols, AtomStringifier};

pub static STRINGIFIER: LazyLock<AtomStringifier> = LazyLock::new(|| -> AtomStringifier { construct_symbols(vec![
    // Built-in atoms
    (000,"∧"), // Conjunction
    (001,"→"), // Implication
    (002,"∀"), // Universal quantiifer
    (003,"¬"), // Negation
    (004,"="), // Identity
    (005,"≠"), // Nonidentity
    (006,"⟨⟩"), // Verbatim
    (007,"Atomic"), // Verbatim
    (008,"Append"), // Verbatim
    // Not built-in
])});
