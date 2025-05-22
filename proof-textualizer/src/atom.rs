use bimap::BiMap;
use shared::atoms::AtomId;

use crate::Textualizer;

#[derive(Clone)]
pub struct AtomTextualizer {
    symbols: BiMap<AtomId, String>
}

impl AtomTextualizer {
    pub fn new(symbols: BiMap<AtomId, String>) -> Self { Self { symbols } }
}

impl Textualizer<AtomId> for AtomTextualizer {
    fn to_text(&self, atom: &AtomId) -> Result<String,()> {
        match self.symbols.get_by_left(atom) {
            Some(symbol) => Ok(symbol.clone()),
            None => Err(()),
        }
    }

    fn from_text(&self, string: &String) -> Result<AtomId,()> {
        match self.symbols.get_by_right(string) {
            Some(symbol) => Ok(symbol.clone()),
            None => Err(()),
        }
    }
}
