use std::{sync::LazyLock};

use tbl_textualization::{structures::expressions::{special_cases::{SpecialCaseParser, SpecialCaseStringifierSet}, SpecialCase}, Stringifier};

use crate::textualizer_formats::plaintext::expression::EXPR_PATTERN_LEXER;


pub static SPECIAL_CASE_PARSER: LazyLock<Box<dyn Stringifier<SpecialCase>>> = LazyLock::new(|| -> Box<dyn Stringifier<SpecialCase>> {
    let function_rules = vec![
        ("(∧,#a..#b)","(#a.. ∧ ..#b)"),
        ("(∀,#a,#b)","∀#a(#b)"),
        ("(→,#a,#b)", "(#a→#b)"),
        ("(¬,#a)", "¬(#b)"),
        ("(=,#a..#b)", "(#a..=..#b)"),
        ("(⟨⟩,#a)", "⟨#b⟩"),
        ("(⌢,#a..#b)", "(#a..⌢..#b)"),
        ("(⚛,#a)", "⚛(#b)")
    ].iter().map(|(pre,post)| -> Result<SpecialCaseParser,()> { 
        SpecialCaseParser::from_strings(pre,post,EXPR_PATTERN_LEXER.clone())
     }).map(|stringifier| -> Box<dyn Stringifier<SpecialCase>> {
        Box::new(stringifier.unwrap()) as Box<dyn Stringifier<SpecialCase>>
     }).collect();

     Box::new(SpecialCaseStringifierSet::new(function_rules))
});
