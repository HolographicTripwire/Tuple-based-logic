use std::{sync::LazyLock};

use tbl_textualization::{structures::expressions::{special_cases::{SpecialCaseTextualizer, SpecialCaseTextualizerSet}, SpecialCase}, Textualizer};

use super::TBL_LEXER;

pub static SPECIAL_CASES: LazyLock<SpecialCaseTextualizerSet> = LazyLock::new(|| -> SpecialCaseTextualizerSet {
    let function_rules = vec![
        ("(∧,#a..#b)","(#a.. ∧ ..#b)"),
        ("(∀,#a,#b)","∀#a(#b)"),
        ("(→,#a,#b)", "(#a→#b)"),
        ("(¬,#a)", "¬(#b)"),
        ("(=,#a..#b)", "(#a..=..#b)"),
        ("(⟨⟩,#a)", "⟨#b⟩"),
        ("(⌢,#a..#b)", "(#a..⌢..#b)"),
        ("(⚛,#a)", "⚛(#b)")
    ].iter().map(|(pre,post)| -> Result<SpecialCaseTextualizer,()> { 
        SpecialCaseTextualizer::from_strings(pre,post,TBL_LEXER.clone())
     }).map(|textualizer| -> Box<dyn Textualizer<SpecialCase>> {
        Box::new(textualizer.unwrap()) as Box<dyn Textualizer<SpecialCase>>
     }).collect();

     return SpecialCaseTextualizerSet::new(function_rules);
});
