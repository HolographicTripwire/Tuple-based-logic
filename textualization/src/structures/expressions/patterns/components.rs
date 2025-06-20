use parsertools::parsers::{helpers::lazy, Parser};

use crate::{helpers::{parsers::{string_parser, word_parser}, styles::Style}, structures::expressions::patterns::parser::ExprPatternStyle};

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum ExprPatternComponent {
    Constant(String),
    Variable(String),
    Variables((String,String),String),
}
impl ExprPatternComponent {
    pub fn new_const(const_string: &str) -> Self { Self::Constant(const_string.to_string()) }
    pub fn new_var(var_name: &str) -> Self { Self::Variable(var_name.to_string()) }
    pub fn new_vars(from: &str, joiner: &str, to: &str) -> Self { Self::Variables((from.to_string(),to.to_string()),joiner.to_string()) }
    pub (super) fn assign(&self, assignment: &ExprPatternAssignment) -> ExprPatternComponent { match assignment {
            ExprPatternAssignment::Constant => self.clone(),
            ExprPatternAssignment::Variable(var, val) => {
                let ExprPatternComponent::Variable(self_var) = self else { return self.clone() };
                if var == self_var { ExprPatternComponent::Constant(val.clone()) } else { self.clone() }
            }, ExprPatternAssignment::Variables((var1, var2), vals) => {
                let ExprPatternComponent::Variables((self_var1,self_var2), sep) = self else { return self.clone() };
                if (var1 == self_var1) & (var2 == self_var2) { ExprPatternComponent::Constant(vals.join(sep)) } else { self.clone() }
            }
        }
    }
}

impl Style<ExprPatternComponent> for ExprPatternStyle {
    fn stringify(&self, stylable: &ExprPatternComponent) -> String {
        match stylable {
            ExprPatternComponent::Constant(_) => todo!(),
            ExprPatternComponent::Variable(_) => todo!(),
            ExprPatternComponent::Variables(_, _) => todo!(),
        }
    }
}

#[derive(PartialEq,Eq,Clone,Hash,Debug)]
pub enum ExprPatternAssignment {
    Constant,
    Variable(String,String),
    Variables((String,String),Vec<String>)
}
impl ExprPatternAssignment {
    pub fn new_const() -> Self
        { Self::Constant }
    pub fn new_var(var: &str, val: &str) -> Self
        { Self::Variable(var.to_string(), val.to_string()) } 
    pub fn new_vars(var_1: &str, var_2: &str, vals: Vec<&str>) -> Self
        { Self::Variables((var_1.to_string(), var_2.to_string()), vals.iter().map(|s| s.to_string()).collect()) }
}

pub fn pattern_component_parser<'a>(component: ExprPatternComponent) -> Parser<'a,char,ExprPatternAssignment> {
    match component {
        ExprPatternComponent::Constant(str) => const_parser(str),
        ExprPatternComponent::Variable(var) => var_parser(var),
        ExprPatternComponent::Variables(vars, joiner) => vars_parser(vars, joiner),
    }
}
fn const_parser<'a>(str: String) -> Parser<'a,char,ExprPatternAssignment> {
    string_parser(&str).unwrap().map(|_| ExprPatternAssignment::Constant)
}
fn var_parser<'a>(var: String) -> Parser<'a,char,ExprPatternAssignment> {
    word_parser().map(move |val| ExprPatternAssignment::Variable(var.clone(), val))
}
fn vars_parser<'a>(vars: (String, String), joiner: String) -> Parser<'a,char,ExprPatternAssignment> {
    vars_parser_inner(joiner.clone()).map(move |vals| ExprPatternAssignment::Variables(vars.clone(), vals))
}
fn vars_parser_inner<'a>(joiner: String) -> Parser<'a,char,Vec<String>> {
    let single_var = word_parser().map(|val| vec![val]);
    let multi_var = word_parser().then(string_parser(&joiner).unwrap()).then(lazy(move || vars_parser_inner(joiner.clone())))
        .map(|((next,_),mut vars)| { vars.insert(0,next); vars });
    single_var.or(multi_var)
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use crate::{structures::expressions::patterns::components::{pattern_component_parser, ExprPatternAssignment, ExprPatternComponent}, test_helpers::{parse_all_str, parse_str}};

    #[test]
    fn test_assign_with_const_and_const() {
        assert_eq!(
            ExprPatternComponent::new_const("fewhuow").assign(&ExprPatternAssignment::new_const()),
            ExprPatternComponent::new_const("fewhuow")
        )
    }

    #[test]
    fn test_assign_with_const_and_var() {
        assert_eq!(
            ExprPatternComponent::new_const("fewhuow").assign(&ExprPatternAssignment::new_var("fewhuow","dawuhofawohu")),
            ExprPatternComponent::new_const("fewhuow")
        )
    }

    #[test]
    fn test_assign_with_const_and_vars() {
        assert_eq!(
            ExprPatternComponent::new_const("fewhuow").assign(&ExprPatternAssignment::new_vars("fewhuow","fewhuow",vec!["odwfawaio", "groahba0"])),
            ExprPatternComponent::new_const("fewhuow")
        )
    }

    #[test]
    fn test_assign_with_var_and_const() {
        assert_eq!(
            ExprPatternComponent::new_var("grogr").assign(&ExprPatternAssignment::new_const()),
            ExprPatternComponent::new_var("grogr")
        )
    }

    #[test]
    fn test_assign_with_var_and_var_unaligned() {
        assert_eq!(
            ExprPatternComponent::new_const("grogr").assign(&ExprPatternAssignment::new_var("fewhuow","dawuhofawohu")),
            ExprPatternComponent::new_const("grogr")
        )
    }

    #[test]
    fn test_assign_with_var_and_var_aligned() {
        assert_eq!(
            ExprPatternComponent::new_var("grogr").assign(&ExprPatternAssignment::new_var("grogr","dawuhofawohu")),
            ExprPatternComponent::new_const("dawuhofawohu")
        )
    }

    #[test]
    fn test_assign_with_var_and_vars() {
        assert_eq!(
            ExprPatternComponent::new_var("grogr").assign(&ExprPatternAssignment::new_vars("grogr","grogr",vec!["odwfawaio", "groahba0"])),
            ExprPatternComponent::new_var("grogr")
        )
    }

    #[test]
    fn test_assign_with_vars_and_const() {
        assert_eq!(
            ExprPatternComponent::new_vars("kbnfeoji","bdijak","ijoaef").assign(&ExprPatternAssignment::new_const()),
            ExprPatternComponent::new_vars("kbnfeoji","bdijak","ijoaef")
        )
    }

    #[test]
    fn test_assign_with_vars_and_var() {
        assert_eq!(
            ExprPatternComponent::new_vars("kbnfeoji","bdijak","ijoaef").assign(&ExprPatternAssignment::new_var("fewhuow","dawuhofawohu")),
            ExprPatternComponent::new_vars("kbnfeoji","bdijak","ijoaef")
        )
    }

    #[test]
    fn test_assign_with_vars_and_vars_unaligned() {
        assert_eq!(
            ExprPatternComponent::new_vars("kbnfeoji","bdijak","ijoaef").assign(&ExprPatternAssignment::new_vars("fewhuow","fewhuow",vec!["odwfawaio", "groahba0"])),
            ExprPatternComponent::new_vars("kbnfeoji","bdijak","ijoaef")
        )
    }

    #[test]
    fn test_assign_with_vars_and_vars_aligned() {
        assert_eq!(
            ExprPatternComponent::new_vars("kbnfeoji","ijoaef","bdijak").assign(&ExprPatternAssignment::new_vars("kbnfeoji","bdijak",vec!["odwfawaio", "groahba0"])),
            ExprPatternComponent::new_const("odwfawaioijoaefgroahba0")
        )
    }
    
    #[test]
    fn test_parser_with_constant_match() {
        assert_eq!(
            parse_str(
                pattern_component_parser(ExprPatternComponent::new_const("Hello")), 
                "Hello"
            ), Ok(ExprPatternAssignment::new_const())
        )
    }
    
    #[test]
    fn test_parser_with_constant_nonmatch() {
        assert!(
            parse_str(
                pattern_component_parser(ExprPatternComponent::new_const("Hello")), 
                "Hello there"
            ).is_err()
        )
    }

    #[test]
    fn test_parser_with_var() {
        assert_eq!(
            parse_str(pattern_component_parser(
                ExprPatternComponent::new_var("Marco")),
                "Polo"
            ), Ok(ExprPatternAssignment::new_var("Marco","Polo"))
        )
    }

    #[test]
    fn test_parser_with_vars_1() {
        assert_eq!(
            parse_str(pattern_component_parser(
                ExprPatternComponent::new_vars("A"," and ","B")),
                "Sugar"
            ), Ok(ExprPatternAssignment::new_vars("A","B",vec!["Sugar"]))
        )
    }

    #[test]
    fn test_parser_with_vars_2() {
        assert_eq!(
            parse_all_str(
                pattern_component_parser(ExprPatternComponent::new_vars("A"," and ","B")),
                "Sugar and Spice"
            ), HashSet::from([
                ExprPatternAssignment::new_vars("A","B",vec!["Sugar","Spice"]),
                ExprPatternAssignment::new_vars("A","B",vec!["Sugar and Spice"])
            ])
        )
    }

    #[test]
    fn test_parser_with_vars_3() {
        assert_eq!(
            parse_all_str(
                pattern_component_parser(ExprPatternComponent::new_vars("A"," and ","B")),
                "Sugar and Spice and Everything nice"
            ), HashSet::from([
                ExprPatternAssignment::new_vars("A","B",vec!["Sugar","Spice", "Everything nice"]),
                ExprPatternAssignment::new_vars("A","B",vec!["Sugar and Spice", "Everything nice"]),
                ExprPatternAssignment::new_vars("A","B",vec!["Sugar","Spice and Everything nice"]),
                ExprPatternAssignment::new_vars("A","B",vec!["Sugar and Spice and Everything nice"])
            ])
        )
    }
}
