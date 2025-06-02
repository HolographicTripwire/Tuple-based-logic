use std::{sync::LazyLock};

use tbl_stringification::{structures::expressions::{special_cases::SpecialCaseStringifierSet, PatternStringifier2, SpecialCase}, Stringifier};

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
    ].iter().map(|(pre,post)| -> Result<PatternStringifier2,()> { 
        PatternStringifier2::from_strings(pre,post,STRINGIFIER_CONTROLS.clone())
     }).map(|stringifier| -> Box<dyn Stringifier<SpecialCase>> {
        Box::new(stringifier.unwrap()) as Box<dyn Stringifier<SpecialCase>>
     }).collect();

     return SpecialCaseStringifierSet::new(function_rules);
});
