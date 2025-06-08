use dyn_clone::DynClone;
use tbl_structures::propositions::Expression;

use crate::{structures::expressions::patterns::{expr_pattern::{ExprPattern, ExprPatternLexer}, ExprPatternParser}, Destringify, Stringify, Stringifier};

#[derive(Clone)]
pub struct SpecialCase { 
    pub expr_components: Vec<Expression>,
    pub string_components: Vec<String>,
    pub vecified_whole: String,
}

#[derive(Clone)]
pub struct SpecialCaseParser(ExprPattern,ExprPattern);

impl SpecialCaseParser {
    pub fn from_strings(pre: &str, post: &str, lexer: Box<ExprPatternLexer>) -> Result<Self,()> {
        let pattern_parser = ExprPatternParser::new(lexer);
        Ok(Self(
            pattern_parser.destringify(&pre.to_string())?,
            pattern_parser.destringify(&post.to_string())?
        ))
    }
}

impl Stringifier<SpecialCase> for SpecialCaseParser {}
impl Stringify<SpecialCase>  for SpecialCaseParser{
    fn stringify(&self, object: &SpecialCase) -> Result<String,()> {
        let string = object.vecified_whole.clone();
        let Ok(replacements) = self.0.match_string(string) else { return Err(()) };
        let pattern = self.1.replace_variables(replacements)?;
        pattern.try_into()
    }
}
impl Destringify<SpecialCase> for SpecialCaseParser {
    fn destringify(&self, string: &String) -> Result<SpecialCase,()> {
        let Ok(replacements) = self.1.match_string(string.clone()) else { return Err(()) };
        let pattern = self.0.replace_variables(replacements)?;
        Ok(SpecialCase {
            expr_components: Vec::new(),
            string_components: Vec::new(),
            vecified_whole: pattern.try_into()?,
        })
    }
}

#[derive(Default,Clone)]
pub struct SpecialCaseStringifierSet {
    cases: Vec<Box<dyn Stringifier<SpecialCase>>>,
}

impl SpecialCaseStringifierSet {
    pub fn new(cases: Vec<Box<dyn Stringifier<SpecialCase>>>) -> Self { Self { cases } }
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
