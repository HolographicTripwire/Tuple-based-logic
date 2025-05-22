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

    fn from_text(&self, string: &String) -> Result<Term,()> {
        let atom_result = self.atoms.from_text(string);
        let tuple_result = self.vecs.from_text(string);
        let optional_rules_result = self.optional_rules.from_text(string);
        
        let ok_results = (atom_result.is_ok() as u8) + (tuple_result.is_ok() as u8) + (optional_rules_result.is_ok() as u8);
        if ok_results < 1 { Err(()) } else if ok_results > 1 { Err(()) }
        
        else if let Ok(atom) = atom_result { Ok(Term::Atomic(atom)) }
        else if let Ok(strings) = tuple_result {
            let terms: Result<Vec<Term>,()> = strings.iter()
                .map(|s| -> Result<Term,()> { self.from_text(s) }).collect();
            Ok(Term::Tuple(terms?)) 
        } else if let Ok((terms, _strings)) = optional_rules_result { Ok(Term::Tuple(terms)) }
        else { Err(()) }
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

pub struct NoRulesTextualizer();

impl Textualizer<(Vec<Term>,Vec<String>)> for NoRulesTextualizer {
    fn to_text(&self, _: &(Vec<Term>,Vec<String>)) -> Result<String,()> {
        Err(())
    }

    fn from_text(&self, _: &String) -> Result<(Vec<Term>,Vec<String>),()> {
        Err(())
    }
}
