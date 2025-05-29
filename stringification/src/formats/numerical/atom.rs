use tbl_structures::atoms::AtomId;

use crate::{Destringify, Stringifier, Stringify};

pub struct NumAtomStringifier();

impl Stringifier<AtomId> for NumAtomStringifier {}
impl Stringify<AtomId> for NumAtomStringifier {
    fn stringify(&self, e: &AtomId) -> Result<String,()> {
        Ok(e.0.0.to_string())
    }
}
impl Destringify<AtomId> for NumAtomStringifier {
    fn destringify(&self, s: &String) -> Result<AtomId,()> {
        let Ok(u) = s.parse::<usize>() else { return Err(()) };
        let Ok(atom) = AtomId::try_from(u) else { return Err(()) };
        Ok(atom)
    }
}
