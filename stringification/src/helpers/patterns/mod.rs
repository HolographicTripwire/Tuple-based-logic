use either::Either;
use enum_iterator::Sequence;
use replacements::ExprPatternReplacements;

use crate::{helpers::lexing::Lexer, structures::{TblStringifierControl, TblStringifierControls}, Destringify};

use super::lexing::Token;


pub mod replacements;
pub mod stringifier;

#[derive(Clone,PartialEq,Eq)]
pub enum ExprPatternComponent {
    Constant(String),
    Variable(String),
    Variables((String,String),String),
}

pub struct ExprPattern{
    components: Vec<ExprPatternComponent>,
    controls: Box<TblStringifierControls>,
}
impl ExprPattern {
    /// Create a new ExprPattern
    fn new(components: Vec<ExprPatternComponent>, controls: Box<TblStringifierControls>) -> Self {
        let mut new_components = Vec::new();
        // Iterate through the provided components
        for component in &components {
            match component {
                // For any ExprPatternComponent::Constant objects, we should join them together if they are consecutive
                ExprPatternComponent::Constant(new_string) => {
                    let combined_string = if let Some(ExprPatternComponent::Constant(old_string)) = components.last()
                        { new_components.pop(); old_string.to_string() + new_string } 
                    else { new_string.to_string() };
                    
                    new_components.push(ExprPatternComponent::Constant(combined_string));
                }, // For any ExprPattern::Variable components, just add them directly without modification
                ExprPatternComponent::Variable(_) => new_components.push(component.clone()),
                // For any ExprPattern::Variables components, just add them direcly without modification
                ExprPatternComponent::Variables((_, _), _) => new_components.push(component.clone()),
            }
        }
        // Create the struct from the components we created
        Self {
            components: new_components,
            controls
        }
    }

    pub fn replace_variables(&self, replacements: ExprPatternReplacements) -> Result<Self,()> {
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
        Ok(ExprPattern { controls: self.controls.clone(), components })
    }

    pub fn match_string(&self, string: String) -> Result<ExprPatternReplacements,()> {
        // Get the control sequence
        let mut control_sequence = self.controls.destringify(&string)?;
        // Create a new map
        let mut map = ExprPatternReplacements::new();
        
        for component in self.components.clone() {
            match component {
                ExprPatternComponent::Constant(s1) => {
                    let Some(Either::Right(s2)) = control_sequence.0.pop() else { return Err(()) };
                    if s1 != s2 { return Err(()) }
                }, ExprPatternComponent::Variable(var) => {
                    let Some(Either::Left(TblStringifierControl::Pattern(ExprPatternControl::VariableIndicator))) = control_sequence.0.pop() else { return Err(()) };
                    let Some(Either::Right(val)) = control_sequence.0.pop() else { return Err(()) };
                    map.add_var_to_val(var,val)?;
                },
                ExprPatternComponent::Variables((var1, var2), sep) => {
                    let Some(Either::Left(TblStringifierControl::Pattern(ExprPatternControl::VariableIndicator))) = control_sequence.0.pop() else { return Err(()) };
                    let Some(Either::Left(TblStringifierControl::Pattern(ExprPatternControl::VariableEnumerator))) = control_sequence.0.pop() else { return Err(()) };
                    let Some(Either::Left(TblStringifierControl::Pattern(ExprPatternControl::VariableEnumerator))) = control_sequence.0.pop() else { return Err(()) };
                    let Some(Either::Left(TblStringifierControl::Pattern(ExprPatternControl::VariableIndicator))) = control_sequence.0.pop() else { return Err(()) };
                    
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
pub enum ExprPatternControl { VariableIndicator, VariableEnumerator }
impl Token for ExprPatternControl {}

#[derive(Clone)]
pub struct ExprPatternControls {
    escape_string: String,
    variable_indicator: String,
    variable_enumerator: String,
}
impl ExprPatternControls {
    pub fn new(escape_string: String, variable_indicator: String, variable_enumerator: String) -> Self 
        { Self { escape_string, variable_indicator, variable_enumerator } }
}
impl Lexer<ExprPatternControl> for ExprPatternControls {
    fn string_from_control(&self, control: &ExprPatternControl) -> &String { match control {
        ExprPatternControl::VariableIndicator => &self.variable_indicator,
        ExprPatternControl::VariableEnumerator => &self.variable_enumerator,
    }}
    
    fn escape_string(&self) -> &String { &self.escape_string }
}
impl Default for ExprPatternControls {
    fn default() -> Self { Self {
        escape_string: "\\".to_string(),
        variable_indicator: "#".to_string(),
        variable_enumerator: "..".to_string(),
    }}
}
