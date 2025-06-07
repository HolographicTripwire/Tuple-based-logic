use tbl_structures::propositions::Expression;

use crate::{structures::{expressions::patterns::{expr_pattern::ExprPattern, ExprPatternTextualizer}, TblLexer}, Detextualize, Textualize, Textualizer};

#[derive(Clone)]
pub struct SpecialCase { 
    pub expr_components: Vec<Expression>,
    pub string_components: Vec<String>,
    pub vecified_whole: String,
}

pub struct SpecialCaseTextualizer(ExprPattern,ExprPattern);

impl SpecialCaseTextualizer {
    pub fn from_strings(pre: &str, post: &str, lexer: Box<TblLexer>) -> Result<Self,()> {
        let pattern_textualizer = ExprPatternTextualizer::new(lexer);
        Ok(Self(
            pattern_textualizer.detextualize(&pre.to_string())?,
            pattern_textualizer.detextualize(&post.to_string())?
        ))
    }
}

impl Textualizer<SpecialCase> for SpecialCaseTextualizer {}
impl Textualize<SpecialCase>  for SpecialCaseTextualizer{
    fn textualize(&self, object: &SpecialCase) -> Result<String,()> {
        let string = object.vecified_whole.clone();
        let Ok(replacements) = self.0.match_string(string) else { return Err(()) };
        let pattern = self.1.replace_variables(replacements)?;
        pattern.try_into()
    }
}
impl Detextualize<SpecialCase> for SpecialCaseTextualizer {
    fn detextualize(&self, string: &String) -> Result<SpecialCase,()> {
        let Ok(replacements) = self.1.match_string(string.clone()) else { return Err(()) };
        let pattern = self.0.replace_variables(replacements)?;
        Ok(SpecialCase {
            expr_components: Vec::new(),
            string_components: Vec::new(),
            vecified_whole: pattern.try_into()?,
        })
    }
}

#[derive(Default)]
pub struct SpecialCaseTextualizerSet {
    cases: Vec<Box<dyn Textualizer<SpecialCase>>>,
}

impl SpecialCaseTextualizerSet {
    pub fn new(cases: Vec<Box<dyn Textualizer<SpecialCase>>>) -> Self { Self { cases } }
}

impl Textualizer<SpecialCase> for SpecialCaseTextualizerSet {}
impl Textualize<SpecialCase> for SpecialCaseTextualizerSet {
    fn textualize(&self, case: &SpecialCase) -> Result<String,()> { 
        let interpretations: Vec<String> = self.cases.iter()
            .filter_map(|textualizer| -> Option<String>{
                match textualizer.textualize(case) {
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
impl Detextualize<SpecialCase> for SpecialCaseTextualizerSet {
    fn detextualize(&self, string: &String) -> Result<SpecialCase,()> { 
        let interpretations: Vec<SpecialCase> = self.cases.iter()
            .filter_map(|textualizer| -> Option<SpecialCase>{
                match textualizer.detextualize(string) {
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
