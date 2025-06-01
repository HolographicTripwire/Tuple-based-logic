use replacements::ExprPatternReplacements;

use crate::{helpers::controls::{Controls, StringifierControl}};

use super::controls::StringifierControls;

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
    controls: Box<StringifierControls>,
}
impl ExprPattern {
    /// Create a new ExprPattern
    fn new(components: Vec<ExprPatternComponent>, controls: Box<StringifierControls>) -> Self {
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
        let mut map = ExprPatternReplacements::new();
        let mut escaping  = false;
        for component in self.components {
            if let Some((c,s)) = self.controls.string_starts_with(&string)? {
                if escaping {escaping = false}
                else {
                    string.drain(0..s.len());
                    match c {
                        StringifierControl::Escape => escaping = true,
                        StringifierControl::Pattern(pattern_control) => todo!(),
                        _ => { return Err(()) }
                    }
                }
            }
            match component {
                ExprPatternComponent::Constant(s) => {
                    if string.starts_with(&s) { string = string[s.len()..string.len()].to_string() } 
                    else { return Err(()) }
                }, ExprPatternComponent::Variable(v) => { 
                    if string.starts_with(self.controls.)
                    map.insert(v,strings[0]);
                },
                ExprPatternComponent::Variables((var1, var2), sep) => {

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
