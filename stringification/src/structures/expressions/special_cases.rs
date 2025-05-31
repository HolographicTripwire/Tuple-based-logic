use tbl_structures::propositions::Expression;

use crate::{Destringify, Stringifier, Stringify};

#[derive(Clone)]
pub struct SpecialCase(pub Vec<Expression>,pub Vec<String>);

pub struct SpecialCaseStringifierSet {
    cases: Vec<Box<dyn Stringifier<SpecialCase>>>,
}

impl SpecialCaseStringifierSet {
    fn new(cases: Vec<Box<dyn Stringifier<SpecialCase>>>) -> Self { Self { cases } }
}

impl Stringifier<SpecialCase> for SpecialCaseStringifierSet {}
impl Stringify<SpecialCase> for SpecialCaseStringifierSet {
    fn stringify(&self, case: &SpecialCase) -> Result<String,()> { 
        let interpretations: Vec<String> = self.cases.iter()
            .filter_map(|stringifier| -> Option<String>{
                match stringifier.stringify(case) {
                    Ok(string) => Some(string),
                    Err(_) => None,
                }})
            .collect();
        
        // Throw an error if this string has multiple valid interpretations
        if interpretations.len() > 1 { Err(()) }
        // If there is only a single valid interpretation, use that one
        else if let Some(string) = interpretations.get(0) { Ok(string.clone()) }
        // Throw an error if this string has no valid interpretations
        else { Err(()) }
    }
}
impl Destringify<SpecialCase> for SpecialCaseStringifierSet {
    fn destringify(&self, string: &String) -> Result<SpecialCase,()> { 
        let interpretations: Vec<SpecialCase> = self.cases.iter()
            .filter_map(|stringifier| -> Option<SpecialCase>{
                match stringifier.destringify(string) {
                    Ok(case) => Some(case),
                    Err(_) => None,
                }})
            .collect();
        
        // Throw an error if this string has multiple valid interpretations
        if interpretations.len() > 1 { Err(()) }
        // If there is only a single valid interpretation, use that one
        else if let Some(case) = interpretations.get(0) { Ok(case.clone()) }
        // Throw an error if this string has no valid interpretations
        else { Err(()) }
    }
}
