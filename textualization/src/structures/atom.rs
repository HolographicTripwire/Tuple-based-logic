use bimap::{BiHashMap, BiMap};
use tbl_structures::atoms::AtomId;

use crate::{Detextualize, Textualizer, Textualize};

#[derive(Clone)]
pub struct AtomTextualizer {
    symbols: BiMap<AtomId, String>
}

impl AtomTextualizer {
    pub fn from_strings(symbols: Vec<(usize,&str)>) -> Self { 
        Self { symbols: BiHashMap::from_iter(
            symbols.iter()
            .map(|(int,str)| -> (AtomId, String) {
                (AtomId::try_from(*int).expect("Atom id out of range when constructing symbols"), str.to_string())
            })
        )}
    }
}

impl Textualizer<AtomId> for AtomTextualizer {}
impl Textualize<AtomId> for AtomTextualizer {
    fn textualize(&self, atom: &AtomId) -> Result<String,()> {
        match self.symbols.get_by_left(atom) {
            Some(symbol) => Ok(symbol.clone()),
            None => Err(()),
        }
    }
}
impl Detextualize<AtomId> for AtomTextualizer {
    fn detextualize(&self, string: &String) -> Result<AtomId,()> {
        match self.symbols.get_by_right(string) {
            Some(symbol) => Ok(symbol.clone()),
            None => Err(()),
        }
    }
}
