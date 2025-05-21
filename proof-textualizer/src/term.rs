use std::collections::HashMap;

use shared::{atoms::AtomId, propositions::Term};

use crate::Textualizer;

pub struct TermTextualizer {
    atoms: Box<dyn Textualizer<AtomId>>,
    vecs: Box<dyn Textualizer<Vec<String>>>,
    optional_rules: Box<dyn Textualizer<(Vec<Term>,Vec<String>)>>,
}

impl TermTextualizer {
    pub fn new(atoms: Box<dyn Textualizer<AtomId>>, vecs: Box<dyn Textualizer<Vec<String>>>, optional_rules: Box<dyn Textualizer<(Vec<Term>,Vec<String>)>>) -> Self {
        Self {atoms, vecs, optional_rules}
    }
}

impl Textualizer<Term> for TermTextualizer {
    fn to_text(&self, term: &Term) -> Result<String,()> {
        match term {
            Term::Atomic(atom_id) => self.atoms.to_text(atom_id),
            Term::Tuple(terms) => {
                let strings: Result<Vec<String>,()> = terms.iter().map(|term| -> Result<String,()> { self.to_text(term) }).collect();
                let term_and_string = (terms.clone(),strings?);
                if let Ok(string) = self.optional_rules.to_text(&term_and_string) { Ok(string) }
                else { self.vecs.to_text(&term_and_string.1) }
            },
        }
    }

    fn from_text(&self, s: &String) -> Result<Term,()> {
        todo!()
    }
}

pub struct FunctionTextualizer {
    map: HashMap<Term,Box<dyn Textualizer<Vec<String>>>>
}

impl Textualizer<(Vec<Term>,Vec<String>)> for FunctionTextualizer {
    fn to_text(&self, (terms,term_strings): &(Vec<Term>,Vec<String>)) -> Result<String,()> {
        let function_head = terms.get(0).ok_or(())?;
        let function_body = &term_strings.iter().skip(1).cloned().collect();
        let textualizer = self.map.get(function_head).ok_or(())?;
        textualizer.to_text(function_body)
    }

    fn from_text(&self, s: &String) -> Result<(Vec<Term>,Vec<String>),()> {
        todo!()
    }
}
