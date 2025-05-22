use std::{sync::LazyLock};

use tuple_based_logic_textualizer::{atom::AtomTextualizer, term::{NoRulesTextualizer, TermTextualizer}};

use super::{construct_symbols, VecTextualizer};

static SYMBOL_TEXTUALIZER: LazyLock<AtomTextualizer> = LazyLock::new(|| -> AtomTextualizer { construct_symbols(vec![
    (0,"")
])});

pub static TERM_TEXTUALIZER: LazyLock<TermTextualizer> = LazyLock::new(|| -> TermTextualizer { TermTextualizer::new(
    Box::new(SYMBOL_TEXTUALIZER.clone()),
    Box::new(VecTextualizer()),
    Box::new(NoRulesTextualizer())
)});
