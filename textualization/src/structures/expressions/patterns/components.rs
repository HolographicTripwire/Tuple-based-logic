use parsertools::{dynamic, pred, Parser};

use crate::helpers::{string_parser, word_parser};

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

#[derive(PartialEq,Eq,Clone,Hash,Debug)]
enum ExprPatternAssignment {
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

fn pattern_component_parser<'a>(component: ExprPatternComponent) -> Parser<'a,char,ExprPatternAssignment> {
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
    vars_parser_inner(joiner).map(move |vals| ExprPatternAssignment::Variables(vars.clone(), vals))
}
fn vars_parser_inner<'a>(joiner: String) -> Parser<'a,char,Vec<String>> {
    let single_var = word_parser().map(|val| vec![val]);
    let multi_var = word_parser().then(string_parser(&joiner).unwrap()).then(dynamic(move || vars_parser_inner(joiner.clone())))
        .map(|((next,_),mut vars)| { vars.insert(0,next); vars });
    single_var.or(multi_var)
}
