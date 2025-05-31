use std::{collections::HashMap, sync::LazyLock};

use tbl_structures::propositions::Expression;

pub static SPECIAL_CASES: LazyLock<SpecialCaseStringifier> = LazyLock::new(|| -> SpecialCaseStringifier {
    SpecialCaseStringifier::from_strings(STRINGIFIER_CONTROLS, vec![
        ("(∧,#a..#b)","(#a.. ∧ ..#b)"),
        ("(∀,#a,#b)","∀#a(#b)"),
        ("(→,#a,#b)", "(#a→#b)"),
        ("(¬,#a)", "¬(#b)"),
        ("(=,#a..#b)", "(#a..=..#b)"),
        ("(⟨⟩,#a)", "⟨#b⟩"),
        ("(⌢,#a..#b)", "(#a..⌢..#b)"),
        ("(⚛,#a)", "⚛(#b)")
    ])
});
