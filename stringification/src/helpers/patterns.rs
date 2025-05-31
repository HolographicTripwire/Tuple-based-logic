use std::collections::HashMap;

use crate::{Destringify, Stringifier, Stringify};

use super::controls::StringifierControls;

pub struct ExprPattern(Vec<ExprPatternComponent>);
impl ExprPattern {
    fn replace_variable(&self, var: String, string: String) -> Result<Self,()> {
        if !self.0.contains(&ExprPatternComponent::Variable(var.clone())) { Err(()) }
        else { Ok(ExprPattern(
            self.0
                .iter()
                .map(|part| -> ExprPatternComponent {
                    if part == &ExprPatternComponent::Variable(var.clone()) { ExprPatternComponent::Constant(string.clone()) }
                    else { part.clone() }       
                })
                .collect()
            ))
        }
    }

    fn match_strings(&self, strings: Vec<String>) -> Result<HashMap<String,String>,()> {
        let mut map = HashMap::new();
        match (self, strings.len() == 1) {
            (ExprPatternComponent::Constant(s), true) => if s != &strings[0] { return Err(()) },
            (ExprPatternComponent::Variable(v), true) => { map.insert(v,strings[0]); },
            ExprPatternComponent::Variables(s_pre, s_in, s_post) => todo!(),
            
        };
        return Ok(map);
    }
}

#[derive(Clone,PartialEq,Eq)]
pub enum ExprPatternComponent {
    Constant(String),
    Variable(String),
    Variables(String,Option<String>,String),
}

pub struct ExprPatternStringifier {
    controls: Box<StringifierControls>
}
impl ExprPatternStringifier {
    pub fn new(controls: Box<StringifierControls>) -> Self {
        Self { controls }
    }
}

impl Stringifier<ExprPattern> for ExprPatternStringifier {}
impl Stringify<ExprPattern> for ExprPatternStringifier {
    fn stringify(&self, object: &ExprPattern) -> Result<String,()> {
        todo!()
    }
}
impl Destringify<ExprPattern> for ExprPatternStringifier {
    fn destringify(&self, string: &String) -> Result<ExprPattern,()> {
        // Get control strings
        let escape_string = &self.controls.escape_string;
        let pattern_controls = &self.controls.pattern_controls;

        todo!()
    }
}