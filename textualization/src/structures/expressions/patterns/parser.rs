use parsertools::{lazy, Parser};

use crate::{helpers::{string_parser, vec_concat_parser, word_parser}, structures::expressions::patterns::{components::ExprPatternComponent, ExprPattern}};

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub struct ExprPatternControls {
    var_indic: String,
    var_enum: String
}
impl ExprPatternControls {
    pub fn new(var_indic: String, var_enum: String) -> Self { Self { var_indic, var_enum } }
    pub fn from_strs(var_indic: &str, var_enum: &str) -> Self
        { Self { var_indic: var_indic.to_string(), var_enum: var_enum.to_string() } }
    
    pub fn var_indic(&self) -> &String { &self.var_indic }
    fn var_indic_parser<'a>(&self) -> Parser<'a,char,()> { string_parser(&self.var_indic).unwrap() }
    pub fn var_enum(&self) -> &String { &self.var_enum }
    fn var_enum_parser<'a>(&self) -> Parser<'a,char,()> { string_parser(&self.var_enum).unwrap() }
}

fn const_parser<'a>(controls: &ExprPatternControls) -> Parser<'a, char, Vec<ExprPatternComponent>>
    { word_parser().map(|s| vec![ExprPatternComponent::Constant(s)]) }
fn var_parser<'a>(controls: &ExprPatternControls) -> Parser<'a, char, Vec<ExprPatternComponent>> {
    let var_indic_parser = controls.var_indic_parser();
    var_indic_parser.clone()
        .then(word_parser())
        .then(var_indic_parser)
        .map(|((_,s),_)| vec![ ExprPatternComponent::Variable(s)])
}
fn vars_parser<'a>(controls: &ExprPatternControls) -> Parser<'a, char, Vec<ExprPatternComponent>> {
    let var_indic_parser = controls.var_indic_parser();
    let var_enum_parser = controls.var_enum_parser();
    var_indic_parser.clone()
        .then(word_parser()).then(var_enum_parser.clone())
        .then(word_parser()).then(var_enum_parser)
        .then(word_parser()).then(var_indic_parser)
        .map(|((((((_,v1),_),sep),_),v2),_)| vec![ExprPatternComponent::Variables((v1,v2),sep)])
}
