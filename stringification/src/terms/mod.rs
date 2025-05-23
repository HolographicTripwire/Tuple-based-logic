mod functions;

pub use functions::FunctionTextualizer;


use tbl_structures::{atoms::AtomId, propositions::Term};

use crate::Textualizer;

/// A textualizer for Terms which builds up Strings from their atomic parts, while applying rules in special cases
pub struct TermTextualizer {
    atoms: Box<dyn Textualizer<AtomId>>,
    vecs: Box<dyn Textualizer<Vec<String>>>,
    special_cases: Box<dyn Textualizer<(Vec<Term>,Vec<String>)>>,
}

impl TermTextualizer {
    pub fn new(atoms: Box<dyn Textualizer<AtomId>>, vecs: Box<dyn Textualizer<Vec<String>>>, special_cases: Box<dyn Textualizer<(Vec<Term>,Vec<String>)>>) -> Self {
        Self {atoms, vecs, special_cases}
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
                if let Ok(string) = self.special_cases.to_text(&term_and_string) { Ok(string) }
                // Otherwise just treat this vec as we would any other vec
                else { self.vecs.to_text(&term_and_string.1) }
            },
        }
    }

    fn from_text(&self, string: &String) -> Result<Term,()> {
        // Try to interpret the provided string with each of our inner textualizers
        let atom_result = self.atoms.from_text(string);
        let tuple_result = self.vecs.from_text(string);
        let optional_rules_result = self.special_cases.from_text(string);
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

/// A rule textualizer that always returns Err(())
pub struct NoRulesTextualizer();

impl Textualizer<(Vec<Term>,Vec<String>)> for NoRulesTextualizer {
    fn to_text(&self, _: &(Vec<Term>,Vec<String>)) -> Result<String,()> { Err(()) }
    fn from_text(&self, _: &String) -> Result<(Vec<Term>,Vec<String>),()> { Err(()) }
}
