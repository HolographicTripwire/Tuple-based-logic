use either::Either;

use crate::{structures::expressions::patterns::{lexer::{ExprPatternLexer, ExprPatternToken}, variable_assignments::VariableAssignments}, Destringify};

pub mod lexer;
pub mod parser;
pub mod variable_assignments;

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
        let mut token_sequence = self.lexer.destringify(&string)?;
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

#[cfg(test)]
mod tests {
    use std::sync::LazyLock;

    use crate::structures::expressions::patterns::parser::ExprPatternParser;

    use super::*;

    const TEST_LEXER: LazyLock<Box<ExprPatternLexer>> = LazyLock::new(|| -> Box<ExprPatternLexer> {
        TEST_PARSER.get_lexer().clone()
    });
    const TEST_PARSER: LazyLock<Box<ExprPatternParser>> = LazyLock::new(|| -> Box<ExprPatternParser> {
        Box::new(ExprPatternParser::default())
    });

    #[test]
    fn test_component_new_const() {
        let const_str = "agejoi23";
        let component = ExprPatternComponent::new_const(const_str);
        assert_eq!(component, ExprPatternComponent::Constant(const_str.to_string()))
    }

    #[test]
    fn test_component_new_var() {
        let var_str = "rheu54w";
        let component = ExprPatternComponent::new_var(var_str);
        assert_eq!(component, ExprPatternComponent::Variable(var_str.to_string()))
    }

    #[test]
    fn test_component_new_vars() {
        let var_left = "feghj6";
        let var_join = "qr23t4y5ui";
        let var_right = "bnmkilu";
        let component = ExprPatternComponent::new_vars(var_left,var_join,var_right);
        assert_eq!(component, ExprPatternComponent::Variables((var_left.to_string(),var_right.to_string()), var_join.to_string()))
    }

    #[test]
    fn test_new_with_single_const_component() {
        let component = ExprPatternComponent::new_const("agejoi23");
        let lexer = Box::new(ExprPatternLexer::default());
        let pattern = ExprPattern::new(vec![component.clone()], lexer.clone());
        assert_eq!(pattern.lexer, TEST_LEXER.clone());
        assert_eq!(pattern.components, vec![component]);
    }

    #[test]
    fn test_new_with_single_var_component() {
        let component = ExprPatternComponent::new_var("rheu54w");
        let pattern = ExprPattern::new(vec![component.clone()], TEST_LEXER.clone());
        assert_eq!(pattern.lexer, TEST_LEXER.clone());
        assert_eq!(pattern.components, vec![component]);
    }

    #[test]
    fn test_new_with_single_vars_component() {
        let component = ExprPatternComponent::new_vars("feghj6","qr23t4y5ui", "bnmkilu");
        let pattern = ExprPattern::new(vec![component.clone()], TEST_LEXER.clone());
        assert_eq!(pattern.lexer, TEST_LEXER.clone());
        assert_eq!(pattern.components, vec![component]);
    }

    #[test]
    fn test_new_with_multiple_const_components() {
        let left_str ="awduge";
        let right_str = "t32u8awd";
        let left_component = ExprPatternComponent::new_const(left_str);
        let right_component = ExprPatternComponent::new_const(right_str);
        let pattern = ExprPattern::new(vec![left_component.clone(), right_component.clone()], TEST_LEXER.clone());
        assert_eq!(pattern.lexer, TEST_LEXER.clone());
        assert_eq!(pattern.components, vec![ExprPatternComponent::Constant(left_str.to_string() + right_str)]);
    }

    #[test]
    fn test_new_with_complex_components() {
        // TODO
    }

    fn construct_assignments(var_mappings: Vec<(&str,&str)>, vars_mappings: Vec<(&str,&str,Vec<&str>)>) -> VariableAssignments {
        let mut assignments = VariableAssignments::new();
        for (k,v) in var_mappings { assignments.add_var_to_val(k.to_string(), v.to_string()); }
        for (k1,k2, vars) in vars_mappings { 
            let new_vars = vars.iter().map(|s| -> String { s.to_string() }).collect();
            assignments.add_vars_to_vals(k1.to_string(), k2.to_string(), new_vars);
        }
        assignments
    }

    fn pre_test_match(pattern_str: &str, match_str: &str, var_mappings: Vec<(&str,&str)>, vars_mappings: Vec<(&str,&str,Vec<&str>)>) -> (Result<VariableAssignments,()>,VariableAssignments) {
        let pattern = TEST_PARSER.destringify(&pattern_str.to_string()).unwrap();
        let assignments = pattern.match_string(match_str.to_string());
        let assingments_check = construct_assignments(var_mappings, vars_mappings);
        return (assignments, assingments_check);
    }
    
    #[test]
    fn test_match_with_const() {
        let (assignments, check) = pre_test_match("r32u89", "r32u89", vec![], vec![]);
        assert_eq!(assignments, Ok(check));
    }

    #[test]
    fn test_match_with_var() {
        let (assignments, check) = pre_test_match("#x1#", "fgt43y4", vec![("x1","fgt43y4")], vec![]);
        assert_eq!(assignments, Ok(check));
    }

    #[test]
    fn test_match_with_vars() {
        let (assignments, check) = pre_test_match("#x1..,..x2#", "a,b,c", vec![], vec![("x1","x2",vec!["a","b","c"])]);
        assert_eq!(assignments, Ok(check));
    }

    #[test]
    fn test_match_with_complex_string() {
        let (assignments, check) = pre_test_match("(#G#,(f,#A.. & ..B#))", "(g_variable,(f,a1 & a2 & a3))", vec![("G","g_variable")], vec![("A","B",vec!["a1","a2","a3"])]);
        assert_eq!(assignments, Ok(check));
    }
}
