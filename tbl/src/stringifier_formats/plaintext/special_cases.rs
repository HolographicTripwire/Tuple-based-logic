use std::{sync::LazyLock};

use tbl_stringification::{structures::expressions::{special_cases::{SpecialCaseStringifier, SpecialCaseStringifierSet}, SpecialCase}, Stringifier};

use super::STRINGIFIER_LEXER;

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
    ].iter().map(|(pre,post)| -> Result<SpecialCaseStringifier,()> { 
        SpecialCaseStringifier::from_strings(pre,post,STRINGIFIER_LEXER.clone())
     }).map(|stringifier| -> Box<dyn Stringifier<SpecialCase>> {
        Box::new(stringifier.unwrap()) as Box<dyn Stringifier<SpecialCase>>
     }).collect();

     return SpecialCaseStringifierSet::new(function_rules);
});
