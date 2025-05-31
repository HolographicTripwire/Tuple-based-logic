use bimap::{BiHashMap, BiMap};
use tbl_structures::atoms::AtomId;

use crate::{Destringify, Stringifier, Stringify};

#[derive(Clone)]
pub struct AtomStringifier {
    symbols: BiMap<AtomId, String>
}

impl AtomStringifier {
    pub fn from_strings(symbols: Vec<(usize,&str)>) -> Self { 
        Self { symbols: BiHashMap::from_iter(
            symbols.iter()
            .map(|(int,str)| -> (AtomId, String) {
                (AtomId::try_from(*int).expect("Atom id out of range when constructing symbols"), str.to_string())
            })
        )}
    }
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
