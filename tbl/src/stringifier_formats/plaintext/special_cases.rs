use std::{sync::LazyLock};

use tbl_stringification::structures::expressions::PatternStringifier2;

use super::STRINGIFIER_CONTROLS;

pub static SPECIAL_CASES: LazyLock<SpecialCaseStringifierSet> = LazyLock::new(|| -> SpecialCaseStringifierSet {
    let function_rules = vec![
        ("(∧,#a..#b)","(#a.. ∧ ..#b)"),
        ("(∀,#a,#b)","∀#a(#b)"),
        ("(→,#a,#b)", "(#a→#b)"),
        ("(¬,#a)", "¬(#b)"),
        ("(=,#a..#b)", "(#a..=..#b)"),
        ("(⟨⟩,#a)", "⟨#b⟩"),
        ("(⌢,#a..#b)", "(#a..⌢..#b)"),
        ("(⚛,#a)", "⚛(#b)")
    ].iter().map(|(pre,post)| -> Box<PatternStringifier2> { 
        Box::new(PatternStringifier2::from_strings(pre,post,STRINGIFIER_CONTROLS))
     }).collect();

     return SpecialCaseStringifierSet::new(function_rules);
});
