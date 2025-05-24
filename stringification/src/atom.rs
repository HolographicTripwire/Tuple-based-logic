use bimap::BiMap;
use tbl_structures::atoms::AtomId;

use crate::{Destringify, Stringifier, Stringify};

#[derive(Clone)]
pub struct AtomStringifier {
    symbols: BiMap<AtomId, String>
}

impl AtomStringifier {
    pub fn new(symbols: BiMap<AtomId, String>) -> Self { Self { symbols } }
}

impl Stringifier<AtomId> for AtomStringifier {}
impl Stringify<AtomId> for AtomStringifier {
    fn stringify(&self, atom: &AtomId) -> Result<String,()> {
        match self.symbols.get_by_left(atom) {
            Some(symbol) => Ok(symbol.clone()),
            None => Err(()),
        }
    }
} 
impl Destringify<AtomId> for AtomStringifier {
    fn destringify(&self, string: &String) -> Result<AtomId,()> {
        match self.symbols.get_by_right(string) {
            Some(symbol) => Ok(symbol.clone()),
            None => Err(()),
        }
    }
}
