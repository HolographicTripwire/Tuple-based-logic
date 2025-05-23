use std::{sync::LazyLock};

use tbl_stringification::{atom::AtomStringifier, terms::{NoRulesStringifier, TermStringifier}};

use super::{construct_symbols, VecStringifier};

static SYMBOL_STRINGIFIER: LazyLock<AtomStringifier> = LazyLock::new(|| -> AtomStringifier { construct_symbols(vec![
    (0,"")
])});

pub static TERM_STRINGIFIER: LazyLock<TermStringifier> = LazyLock::new(|| -> TermStringifier { TermStringifier::new(
    Box::new(SYMBOL_STRINGIFIER.clone()),
    Box::new(VecStringifier()),
    Box::new(NoRulesStringifier())
)});
