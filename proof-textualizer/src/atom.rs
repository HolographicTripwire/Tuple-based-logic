use bimap::BiMap;
use shared::atoms::AtomId;

use crate::Textualizer;

pub struct AtomTextualizer {
    symbols: BiMap<AtomId, String>
}

impl Textualizer<AtomId> for AtomTextualizer {
    fn to_text(&self, atom: &AtomId) -> Result<String,()> {
        match self.symbols.get_by_left(atom) {
            Some(symbol) => Ok(symbol.clone()),
            None => Err(()),
        }
    }

    fn from_text(&self, s: &String) -> Result<AtomId,()> {
        match self.symbols.get_by_right(s) {
            Some(symbol) => Ok(symbol.clone()),
            None => Err(()),
        }
    }
}
