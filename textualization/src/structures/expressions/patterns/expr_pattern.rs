use either::Either;
use enum_iterator::Sequence;
use super::variable_assignments::VariableAssignments;

use crate::{helpers::lexing::{Lexer, Token}, Detextualize};

#[derive(Clone,PartialEq,Eq,Debug)]
pub enum ExprPatternComponent {
    Constant(String),
    Variable(String),
    Variables((String,String),String),
}
impl ExprPatternComponent {
    pub fn new_const(const_string: &str) -> Self { Self::Constant(const_string.to_string()) }
    pub fn new_var(var_name: &str) -> Self { Self::Variable(var_name.to_string()) }
    pub fn new_vars(from: &str, joiner: &str, to: &str) -> Self { Self::Variables((from.to_string(),to.to_string()),joiner.to_string()) }
}

#[derive(Clone,PartialEq,Eq,Debug)]
pub struct ExprPattern{
    components: Vec<ExprPatternComponent>,
    lexer: Box<ExprPatternLexer>,
}
impl ExprPattern {
    /// Create a new ExprPattern
    pub fn new(components: Vec<ExprPatternComponent>, lexer: Box<ExprPatternLexer>) -> Self {
        Self {
            components: Self::remove_redundancy(components),
            lexer
        }
    }

    pub fn get_components(&self) -> &Vec<ExprPatternComponent> { return &self.components }

    fn remove_redundancy(components: Vec<ExprPatternComponent>) -> Vec<ExprPatternComponent> {
        let mut new_components = Vec::new();
        // Iterate through the provided components
        let mut combined_string = "".to_string();
        let push_combined_string = |combined_string: &mut String, new_components: &mut Vec<ExprPatternComponent>| -> () {
            if combined_string.len() > 0 {
                new_components.push(ExprPatternComponent::Constant(combined_string.clone()));
                combined_string.clear();
            }
        };
        for component_i in components {
            if let ExprPatternComponent::Constant(_) = component_i {}
            else { push_combined_string(&mut combined_string, &mut new_components) }
            match component_i {
                // For any ExprPatternComponent::Constant objects, we should join them together if they are consecutive
                ExprPatternComponent::Constant(new_string) => combined_string += &new_string,
                // For any ExprPattern::Variable components, just add them directly without modification
                ExprPatternComponent::Variable(_) => new_components.push(component_i.clone()),
                // For any ExprPattern::Variables components, just add them direcly without modification
                ExprPatternComponent::Variables((_, _), _) => new_components.push(component_i.clone()),
            }
        } push_combined_string(&mut combined_string, &mut new_components);
        new_components
    }

    pub fn replace_variables(&self, replacements: VariableAssignments) -> Result<Self,()> {
        let components = self.components
            .iter()
            .map(|component| -> Result<ExprPatternComponent,()> { match component {
                ExprPatternComponent::Constant(_) => Ok(component.clone()),
                ExprPatternComponent::Variable(var) => match replacements.get_va1_from_var(var) {
                    Some(val) => Ok(ExprPatternComponent::Constant(val.to_string())),
                    None => Ok(component.clone()),
                }, ExprPatternComponent::Variables((var1, var2), sep) => {
                    match (replacements.get_vals_from_vars(var1, var2)) {
                        None => Ok(component.clone()),
                        Some(strings) => Ok(ExprPatternComponent::Constant(strings.join(sep))),
                    }
                },
            }}).collect::<Result<Vec<ExprPatternComponent>,()>>()?;
        Ok(ExprPattern { lexer: self.lexer.clone(), components })
    }

    pub fn match_string(&self, string: String) -> Result<VariableAssignments,()> {
        // Get the token sequence
        let mut token_sequence = self.lexer.detextualize(&string)?;
        // Create a new map
        let mut map = VariableAssignments::new();
        
        for component in self.components.clone() {
            match component {
                ExprPatternComponent::Constant(s1) => {
                    let Some(Either::Right(s2)) = token_sequence.0.pop() else { return Err(()) };
                    if s1 != s2 { return Err(()) }
                }, ExprPatternComponent::Variable(var) => {
                    let Some(Either::Left(ExprPatternToken::VariableIndicator)) = token_sequence.0.pop() else { return Err(()) };
                    let Some(Either::Right(val)) = token_sequence.0.pop() else { return Err(()) };
                    map.add_var_to_val(var,val)?;
                },
                ExprPatternComponent::Variables((var1, var2), sep) => {
                    let Some(Either::Left(ExprPatternToken::VariableIndicator)) = token_sequence.0.pop() else { return Err(()) };
                },
            };
        }
        return Ok(map);
    }
}

impl TryInto<String> for ExprPattern {
    type Error = ();

    fn try_into(self) -> Result<String, Self::Error> {
        let [sole_component] = self.components.as_slice() else { return Err(()) };
        let ExprPatternComponent::Constant(str) = sole_component else { return Err(()) };
        Ok(str.clone())
    }
}

#[derive(Sequence, Clone, Copy)]
pub enum ExprPatternToken { VariableIndicator, VariableEnumerator }
impl Token for ExprPatternToken {}

#[derive(Clone,Debug,PartialEq,Eq)]
pub struct ExprPatternLexer {
    escape_string: String,
    variable_indicator: String,
    variable_enumerator: String,
}
impl ExprPatternLexer {
    pub fn new(escape_string: String, variable_indicator: String, variable_enumerator: String) -> Self 
        { Self { escape_string, variable_indicator, variable_enumerator } }
}
impl Lexer<ExprPatternToken> for ExprPatternLexer {
    fn string_from_token(&self, token: &ExprPatternToken) -> &String { match token {
        ExprPatternToken::VariableIndicator => &self.variable_indicator,
        ExprPatternToken::VariableEnumerator => &self.variable_enumerator,
    }}
    
    fn escape_string(&self) -> &String { &self.escape_string }
}
impl Default for ExprPatternLexer {
    fn default() -> Self { Self {
        escape_string: "\\".to_string(),
        variable_indicator: "#".to_string(),
        variable_enumerator: "..".to_string(),
    }}
}
