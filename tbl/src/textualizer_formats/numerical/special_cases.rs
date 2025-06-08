use std::{sync::LazyLock};

use tbl_textualization::{structures::expressions::{special_cases::{SpecialCaseParser, SpecialCaseStringifierSet}, SpecialCase}, Stringifier};

use crate::textualizer_formats::plaintext::expression::EXPR_PATTERN_LEXER;


pub static SPECIAL_CASE_PARSER: LazyLock<Box<dyn Stringifier<SpecialCase>>> = LazyLock::new(|| -> Box<dyn Stringifier<SpecialCase>> {
     Box::new(SpecialCaseStringifierSet::new(vec![]))
});
