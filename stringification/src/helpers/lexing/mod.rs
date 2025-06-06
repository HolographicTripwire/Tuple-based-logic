use either::Either;
use enum_iterator::{all, Sequence};

use crate::{Destringify, Stringifier, Stringify};

/// This trait allows an enum to be used as a Control for an implementer of the Controls trait
pub trait Token: Sequence + Clone {}

#[derive(Clone)]
pub struct TokenMapping<C: Token>(pub C,pub String);

pub struct TokenSequence<C: Token>(pub Vec<Either<C,String>>);

pub trait Lexer<C>: Clone + Send + Sync {
    fn escape_string(&self) -> &String;
    fn string_from_control(&self, control: &C) -> &String;
}

impl <T: Token, L: Lexer<T>> Stringifier<TokenSequence<T>> for L {}
impl <T: Token, L: Lexer<T>> Stringify<TokenSequence<T>> for L {
    fn stringify(&self, sequence: &TokenSequence<T>) -> Result<String,()> {
        let mut string = "".to_string();
        for s in &sequence.0 { match s {
            Either::Left(c) => string += self.string_from_control(&c),
            Either::Right(s) => string += s.as_str(),
        }}
        Ok(string)
    }
}
impl <T: Token, L: Lexer<T>> Destringify<TokenSequence<T>> for L {
    fn destringify(&self, string: &String) -> Result<TokenSequence<T>,()> {
        let mut sequence: Vec<Either<T, String>> = Vec::new();
        let mut escaping = false;
        let mut current_string = "".to_string();
        for char in string.chars() {
            current_string.push(char);
            if !escaping {
                if let Ok(_) = pop_escape_from_string_end(self,&mut current_string) { 
                    escaping = true;
                } else if let Some(control_string) = pop_token_from_string_end(self, &mut current_string)? {
                    if current_string.len() > 0 { sequence.push(Either::Right(current_string)) }
                    sequence.push(Either::Left(control_string.0));
                    current_string = "".to_owned();
                }
            } else { escaping = false; }
        }
        if current_string.len() > 0 { sequence.push(Either::Right(current_string)) }
        Ok(TokenSequence(sequence))
    }
}

fn pop_escape_from_string_end<T: Token, L: Lexer<T>>(controls: &L, string: &mut String) -> Result<String,()> {
    let escape_string = controls.escape_string();
    if string.ends_with(escape_string) {
        pop_n_from_end(string, escape_string.len());
        Ok(escape_string.clone())
    } else { Err(()) }
}

fn pop_token_from_string_end<T: Token, L: Lexer<T>>(controls: &L, string: &mut String) -> Result<Option<TokenMapping<T>>,()> {
    Ok(match string_ends_with(controls,string)? {
        Some(control_string) => {
            pop_n_from_end(string,control_string.1.len());
            Some(control_string)
        }, None => None,
    })
}

fn pop_n_from_end(string: &mut String, n: usize) -> std::string::Drain<'_> {
    string.drain(string.len()-n..string.len())
}


fn string_ends_with<T: Token, L: Lexer<T>>(controls: &L, string: &String) -> Result<Option<TokenMapping<T>>,()> {
    let ends_with = get_controls_and_strings(controls).iter()
        .filter_map(|TokenMapping(c, s)| -> Option<TokenMapping<T>> 
            { if string.ends_with(s) { Some(TokenMapping(c.clone(),s.clone())) } else { None } }
        ).collect::<Vec<TokenMapping<T>>>();
    // If there are multiple plausible control strings that this string could start with
    if ends_with.len() > 1 { return Err(()) }
    Ok(ends_with.get(0).cloned())
}

fn get_controls<T: Token, L: Lexer<T>>(_: &L) -> Vec<T> { all::<T>().collect::<Vec<T>>() }
fn get_controls_and_strings<T: Token, L: Lexer<T>>(controls: &L) -> Vec<TokenMapping<T>> { 
    get_controls(controls)
        .iter()
        .map(|c| -> TokenMapping<T> { TokenMapping(c.clone(), controls.string_from_control(c).clone()) })
        .collect()
}
