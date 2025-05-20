use shared::{atoms::AtomId, propositions::Term};

use crate::Textualizer;

pub struct TermTextualizer<'a> {
    atoms: &'a dyn Textualizer<AtomId>,
    vecs: &'a dyn Textualizer<Vec<String>>,
    optional_rules: &'a dyn Textualizer<(Term,Vec<String>)>,
}

impl <'a> Textualizer<Term> for TermTextualizer<'a> {
    fn to_text(&self, term: &Term) -> Result<String,()> {
        match term {
            Term::Atomic(atom_id) => self.atoms.to_text(atom_id),
            Term::Tuple(terms) => {
                let strings: Result<Vec<String>,()> = terms.iter().map(|term| -> Result<String,()> { self.to_text(term) }).collect();
                let term_and_string = (term.clone(),strings?);
                if let Ok(string) = self.optional_rules.to_text(&term_and_string) { Ok(string) }
                else { self.vecs.to_text(&term_and_string.1) }
            },
        }
    }

    fn from_text(&self, s: &String) -> Result<Term,()> {
        todo!()
    }
}
