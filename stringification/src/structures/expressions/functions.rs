use std::collections::HashMap;

use tbl_structures::propositions::Expression;

use crate::{Destringify, Stringifier, Stringify};


pub struct FunctionStringifier {
    map: HashMap<Expression,Box<dyn Stringifier<Vec<String>>>>
}

impl Stringify<(Vec<Expression>,Vec<String>)> for FunctionStringifier {
    fn stringify(&self, (exprs,expr_strings): &(Vec<Expression>,Vec<String>)) -> Result<String,()> {
        // If the head of the expression is not a function, return an error
        let function_head = exprs.get(0).ok_or(())?;
        // Get all elements in the vec besides the function head
        let function_body = &expr_strings.iter().skip(1).cloned().collect();
        // Use the function head to textualize the remainder of the function
        self.map.get(function_head)
            .ok_or(())?
            .stringify(function_body)
    }
}
impl Destringify<(Vec<Expression>,Vec<String>)> for FunctionStringifier {
    fn destringify(&self, string: &String) -> Result<(Vec<Expression>,Vec<String>),()> {
        // Get all valid interpretations
        let interpretations: Vec<(&Expression,Vec<String>)> = self.map.iter()
            .filter_map(|(expr, textualizer)| -> Option<(&Expression, Vec<String>)>{
                match textualizer.destringify(string) {
                    Ok(strings) => Some((expr, strings)),
                    Err(_) => None,
                }})
            .collect();
        
        // Throw an error if this string has multiple valid interpretations
        if interpretations.len() > 1 { Err(()) }
        // If there is only a single valid interpretation, use that one
        else if let Some((expr, strings)) = interpretations.get(0) { Ok((vec![(*expr).clone()],strings.clone())) }
        // Throw an error if this string has no valid interpretations
        else { Err(()) }
    }
}
