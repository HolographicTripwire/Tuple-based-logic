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
                // Convert each term in the tuple to a string
                let strings: Result<Vec<String>,()> = terms.iter().map(|term| -> Result<String,()> { self.to_text(term) }).collect();
                // Pair terms with strings
                let term_and_string = (terms.clone(),strings?);
                
                // If there are any optional rules, apply them
                if let Ok(string) = self.optional_rules.to_text(&term_and_string) { Ok(string) }
                // Otherwise just treat this vec as we would any other vec
                else { self.vecs.to_text(&term_and_string.1) }
            },
        }
    }

    fn from_text(&self, string: &String) -> Result<Term,()> {
        // Try to interpret the provided string with each of our inner textualizers
        let atom_result = self.atoms.from_text(string);
        let tuple_result = self.vecs.from_text(string);
        let optional_rules_result = self.optional_rules.from_text(string);
        // Calculate the number of valid interpretations we found
        let ok_results = (atom_result.is_ok() as u8) + (tuple_result.is_ok() as u8) + (optional_rules_result.is_ok() as u8);
        
        // Throw an error if this string has multiple valid interpretations
        if ok_results > 1 { Err(()) }
        // If there is only a single valid interpretation, use that one
        else if let Ok(atom) = atom_result { Ok(Term::Atomic(atom)) }
        else if let Ok(strings) = tuple_result {
            let terms: Result<Vec<Term>,()> = strings.iter()
                .map(|s| -> Result<Term,()> { self.from_text(s) })
                .collect();
            Ok(Term::Tuple(terms?)) 
        } else if let Ok((_, strings)) = optional_rules_result {
            let terms: Result<Vec<Term>,()> = strings.iter()
                .map(|string| -> Result<Term,()> { self.from_text(string) })
                .collect();
            Ok(Term::Tuple(terms?))
        }
        // Throw an error if this string has no interpretations
        else { Err(()) }
    }
}

pub struct FunctionTextualizer {
    map: HashMap<Term,Box<dyn Textualizer<Vec<String>>>>
}

impl Textualizer<(Vec<Term>,Vec<String>)> for FunctionTextualizer {
    fn to_text(&self, (terms,term_strings): &(Vec<Term>,Vec<String>)) -> Result<String,()> {
        // If the head of the term is not a function, return an error
        let function_head = terms.get(0).ok_or(())?;
        // Get all elements in the vec besides the function head
        let function_body = &term_strings.iter().skip(1).cloned().collect();
        // Use the function head to textualize the remainder of the function
        self.map.get(function_head)
            .ok_or(())?
            .to_text(function_body)
    }

    fn from_text(&self, string: &String) -> Result<(Vec<Term>,Vec<String>),()> {
        // Get all valid interpretations
        let interpretations: Vec<(&Term,Vec<String>)> = self.map.iter()
            .filter_map(|(term, textualizer)| -> Option<(&Term, Vec<String>)>{
                match textualizer.from_text(string) {
                    Ok(strings) => Some((term, strings)),
                    Err(_) => None,
                }})
            .collect();
        
        // Throw an error if this string has multiple valid interpretations
        if interpretations.len() > 1 { Err(()) }
        // If there is only a single valid interpretation, use that one
        else if let Some((term, strings)) = interpretations.get(0) { Ok((vec![(*term).clone()],strings.clone())) }
        // Throw an error if this string has no valid interpretations
        else { Err(()) }
    }
}

/// A rule textualizer that always returns Err(())
pub struct NoRulesTextualizer();

impl Textualizer<(Vec<Term>,Vec<String>)> for NoRulesTextualizer {
    fn to_text(&self, _: &(Vec<Term>,Vec<String>)) -> Result<String,()> { Err(()) }
    fn from_text(&self, _: &String) -> Result<(Vec<Term>,Vec<String>),()> { Err(()) }
}
