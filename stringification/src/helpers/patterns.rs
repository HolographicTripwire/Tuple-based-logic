use std::collections::{HashMap, HashSet};

use crate::{helpers::controls::{Controls, StringifierControl}, Destringify, Stringifier, Stringify};

use super::controls::StringifierControls;

pub struct ExprPatternReplacements {
    vars: HashSet<String>,
    var_to_val: HashMap<String,String>,
    vars_to_vals: HashMap<(String,String),Vec<String>>
}
impl ExprPatternReplacements {
    fn new() -> Self { Self {
        vars: HashSet::new(),
        var_to_val: HashMap::new(), 
        vars_to_vals: HashMap::new()
    }}

    fn add_var_from_val(&mut self, k: String, v: String) -> Result<(),()> {
        if self.vars.contains(&k) { return Err(()); }
        self.vars.insert(k.clone());
        self.var_to_val.insert(k, v);
        return Ok(());
    }
    fn get_va1_from_var(&self, s: &String) -> Option<&String>
        { self.var_to_val.get(s) }

    fn add_vars_from_val(&mut self, k1: String, k2: String, v: Vec<String>) -> Result<(),()> { 
        if self.vars.contains(&k1) || self.vars.contains(&k2) { return Err(()); }
        self.vars_to_vals.insert((k1,k2), v);
        return Ok(());
    }
    fn get_vals_from_vars(&self, s1: &String, s2: &String) -> Option<&Vec<String>> 
        { self.vars_to_vals.get(&(s1.clone(),s2.clone())) }
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

#[derive(Clone,PartialEq,Eq)]
pub enum ExprPatternComponent {
    Constant(String),
    Variable(String),
    Variables((String,String),String),
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
        let escape_string = self.controls.string_from_control(&StringifierControl::Escape);
        let pattern_controls = &self.controls;

        todo!()
    }
}