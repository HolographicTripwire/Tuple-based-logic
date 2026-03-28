use std::{fmt::Display, process::Output};

use nom::Parser;

mod propositions;
mod inferences;

enum ParseError<V,E>{
    CharactersRemaining((V,String)),
    Other(nom::Err<E>)
}
pub trait Style<Sb: Stylable>: Sized {
    type ParseParams;
    type Err;

    fn apply_style(self, stylable: Sb) -> Styled<Sb,Self> { Styled::new(stylable, self) }

    fn format<'a>(&self, f:&mut std::fmt::Formatter<'a>, stylable: &Sb) -> std::fmt::Result;

    fn parser(&self, params: Self::ParseParams) -> impl Parser<&str,Output=Sb,Error=Self::Err>;
    fn parse_all(&self, str: &str, params: Self::ParseParams) -> Result<Sb,ParseError<Sb,Self::Err>> {
        let (remaining,out) = self.parser(params)
            .parse(str)
            .map_err(|e| ParseError::Other(e))?;
        if remaining.len() == 0 { Ok(out) }
        else { Err(ParseError::CharactersRemaining((out, remaining.to_string()))) }
    }
}

pub trait Stylable: Sized {
    fn styled<S: Style<Self>>(self, style: S) -> Styled<Self,S>
        { Styled::new(self, style) }
}
impl <Sb: Clone> Stylable for Sb {}

pub struct Styled<Sb: Stylable, S: Style<Sb>> {
    stylable: Sb,
    style: S
}
impl <Sb: Stylable, S: Style<Sb>> Styled<Sb,S> {
    fn new(stylable: Sb, style: S) -> Self { Self { stylable, style } }
}
impl <Sb: Stylable, S: Style<Sb>> Display for Styled<Sb,S> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.style.format(f, &self.stylable)
    }
}
