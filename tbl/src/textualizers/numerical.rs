use std::{sync::LazyLock};

use shared::atoms::AtomId;
use tuple_based_logic_textualizer::{term::{NoRulesTextualizer, TermTextualizer}, Textualizer};

use super::{VecTextualizer};

struct NumAtomTextualizer();

impl Textualizer<AtomId> for NumAtomTextualizer {
    fn to_text(&self, e: &AtomId) -> Result<String,()> {
        Ok(e.0.0.to_string())
    }

    fn from_text(&self, s: &String) -> Result<AtomId,()> {
        todo!()
    }
}

pub static TERM_TEXTUALIZER: LazyLock<TermTextualizer> = LazyLock::new(|| -> TermTextualizer { TermTextualizer::new(
    Box::new(NumAtomTextualizer()),
    Box::new(VecTextualizer()),
    Box::new(NoRulesTextualizer())
)});
