use std::collections::HashMap;

use bimap::BiMap;
use parsertools::{pred, Parser};
use tbl_structures::atoms::AtomId;

use crate::helpers::{num_parser, string_parser};

#[derive(Clone)]
pub struct AtomControls { ids_and_symbols: BiMap<AtomId,String>, atom_id_indicator: String }
impl AtomControls {
    pub fn new(ids_and_symbols: BiMap<AtomId, String>, atom_id_indicator: String) -> Result<Self,()> {
        Ok(Self { ids_and_symbols, atom_id_indicator })
    }

    pub fn atoms(&self) -> &BiMap<AtomId,String> { &self.ids_and_symbols }
    pub fn atoms_by_id(&self) -> HashMap<AtomId,String> {
        self.ids_and_symbols.left_values()
            .zip(self.ids_and_symbols.right_values())
            .map(|(a,b)| (a.clone(),b.clone()))
            .collect()
    }
    pub fn atoms_by_symbol(&self) -> HashMap<String,AtomId> {
        self.ids_and_symbols.right_values()
            .zip(self.ids_and_symbols.left_values())
            .map(|(a,b)| (a.clone(),b.clone()))
            .collect()
    }

    pub fn id_indicator(&self) -> &String { &self.atom_id_indicator }
}
impl Default for AtomControls {
    fn default() -> Self {
        Self { ids_and_symbols: BiMap::new(), atom_id_indicator: "#".to_string() }
    }
}

pub fn atom_parser<'a>(controls: &AtomControls) -> Parser<'a, char,AtomId> {
    let ids = atom_id_parser(controls.id_indicator()).unwrap();
    let symbols = atom_symbol_parser(controls.atoms_by_id()).unwrap();
    ids.or(symbols)
}

pub fn atom_symbol_parser<'a>(atoms: HashMap<AtomId, String>) -> Result<Parser<'a, char,AtomId>,()> {
    let all_atoms = atoms.iter()
        .map(|(id,string)| -> Result<Parser<char, AtomId>,()> { atom_symbol_parser_inner(*id, string) } )
            .collect::<Result<Vec<Parser<char,AtomId>>,()>>()?;
    let any_atom = all_atoms.into_iter()
            .reduce(|acc, e| acc.or(e));
    Ok(any_atom.unwrap_or(pred(|_| None)))
}

fn atom_symbol_parser_inner<'a>(id: AtomId, string: &String) -> Result<Parser<'a, char,AtomId>,()> {
    Ok(string_parser(string)?
        .map(move |_| id)
    )
}

pub fn atom_id_parser<'a>(atom_id_indicator: &String) -> Result<Parser<'a, char,AtomId>,()> {
    Ok(string_parser(&atom_id_indicator)?.then(
        num_parser()
    ).map(|(_, uint)| -> AtomId { AtomId::try_from(uint).unwrap() }))
}
