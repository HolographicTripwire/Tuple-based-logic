use tbl_structures::atoms::AtomId;

use tbl_textualization::{Detextualize, Textualizer, Textualize};

pub struct NumAtomTextualizer();

impl Textualizer<AtomId> for NumAtomTextualizer {}
impl Textualize<AtomId> for NumAtomTextualizer {
    fn textualize(&self, e: &AtomId) -> Result<String,()> {
        Ok(e.0.0.to_string())
    }
}
impl Detextualize<AtomId> for NumAtomTextualizer {
    fn detextualize(&self, s: &String) -> Result<AtomId,()> {
        let Ok(u) = s.parse::<usize>() else { return Err(()) };
        let Ok(atom) = AtomId::try_from(u) else { return Err(()) };
        Ok(atom)
    }
}
