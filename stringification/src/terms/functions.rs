use std::collections::HashMap;

use tbl_structures::propositions::Term;

use crate::Textualizer;

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
