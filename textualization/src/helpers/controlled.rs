use std::collections::HashSet;

use parsertools::{lazy, Parser};

use crate::helpers::{iter_disjunction_parser_transformer, string_parser, word_parser};

#[derive(Clone)]
pub struct ControlStrings {
    escape: String, 
    others: HashSet<String>
}
impl ControlStrings {
    pub fn escape(&self) -> &String { &self.escape }
    pub fn escape_parser(&self) -> Parser<char,String> { string_parser(&self.escape).unwrap() }
    pub fn others(&self) -> &HashSet<String> { &self.others }
    pub fn others_parser(&self) -> Parser<char,String> { iter_disjunction_parser_transformer(self.others.iter().map(|s| string_parser(s).unwrap())) }
}

pub fn controlled_word_parser<'a>(controls: &'a ControlStrings) -> Parser<'a,char,String> {
    let inner = controlled_word_parser_inner(controls);
    inner.clone().or(inner.then(lazy(|| controlled_word_parser(controls))).map(|(l,r)| l + &r))
}
fn controlled_word_parser_inner<'a>(controls: &'a ControlStrings) -> Parser<'a,char,String> {
    let word_not_containing_control = word_not_containing_parser(controls.others().clone());
    let escaped_control = controls.escape_parser()
        .then(controls.escape_parser().or(controls.others_parser()))
        .map(|(_,s)| s);
    word_not_containing_control.or(escaped_control)
    
}

fn word_not_containing_parser<'a>(set: HashSet<String>) -> Parser<'a, char, String> {
    word_parser().filter(
        move |word| !set.iter().any(|should_not_contain| -> bool { word.contains(should_not_contain) }),
        parsertools::ParseError::UnexpectedTokenProperUnknown
    )
}
