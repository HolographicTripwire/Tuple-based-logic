use std::{sync::LazyLock};

use tbl_stringification::{atom::AtomStringifier, terms::{NoSpecialCasesStringifier, TermStringifier}, Stringifier};
use tbl_structures::propositions::Term;

use super::{construct_symbols, VecStringifier};

static SYMBOL_STRINGIFIER: LazyLock<AtomStringifier> = LazyLock::new(|| -> AtomStringifier { construct_symbols(vec![
    (0,"")
])});

pub static TERM_STRINGIFIER: LazyLock<Box<dyn Stringifier<Term>>> = 
    LazyLock::new(|| -> Box<dyn Stringifier<Term>> { 
        Box::new(TermStringifier::new(
            SYMBOL_STRINGIFIER.clone(),
            VecStringifier(),
            NoSpecialCasesStringifier()
        ))
    });
