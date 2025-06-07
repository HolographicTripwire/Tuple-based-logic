use either::Either;
use enum_iterator::{all, Sequence};

use crate::{Detextualize, Textualizer, Textualize};

/// This trait allows an enum to be used as a Token for an implementer of the Lexer trait
pub trait Token: Sequence + Clone {}

#[derive(Clone)]
pub struct TokenMapping<T: Token>(pub T,pub String);

pub struct TokenSequence<T: Token>(pub Vec<Either<T,String>>);

pub trait Lexer<T>: Clone + Send + Sync {
    fn escape_string(&self) -> &String;
    fn string_from_token(&self, token: &T) -> &String;
}

impl <T: Token, L: Lexer<T>> Textualizer<TokenSequence<T>> for L {}
impl <T: Token, L: Lexer<T>> Textualize<TokenSequence<T>> for L {
    fn textualize(&self, sequence: &TokenSequence<T>) -> Result<String,()> {
        let mut string = "".to_string();
        for s in &sequence.0 { match s {
            Either::Left(c) => string += self.string_from_token(&c),
            Either::Right(s) => string += s.as_str(),
        }}
        Ok(string)
    }
}
impl <T: Token, L: Lexer<T>> Detextualize<TokenSequence<T>> for L {
    fn detextualize(&self, string: &String) -> Result<TokenSequence<T>,()> {
        let mut sequence: Vec<Either<T, String>> = Vec::new();
        let mut escaping = false;
        let mut current_string = "".to_string();
        for char in string.chars() {
            current_string.push(char);
            if !escaping {
                if let Ok(_) = pop_escape_from_string_end(self,&mut current_string) { 
                    escaping = true;
                } else if let Some(token_mapping) = pop_token_from_string_end(self, &mut current_string)? {
                    if current_string.len() > 0 { sequence.push(Either::Right(current_string)) }
                    sequence.push(Either::Left(token_mapping.0));
                    current_string = "".to_owned();
                }
            } else { escaping = false; }
        }
        if current_string.len() > 0 { sequence.push(Either::Right(current_string)) }
        Ok(TokenSequence(sequence))
    }
}

fn pop_escape_from_string_end<T: Token, L: Lexer<T>>(lexer: &L, string: &mut String) -> Result<String,()> {
    let escape_string = lexer.escape_string();
    if string.ends_with(escape_string) {
        pop_n_from_end(string, escape_string.len());
        Ok(escape_string.clone())
    } else { Err(()) }
}

fn pop_token_from_string_end<T: Token, L: Lexer<T>>(lexer: &L, string: &mut String) -> Result<Option<TokenMapping<T>>,()> {
    Ok(match string_ends_with(lexer,string)? {
        Some(token_mapping) => {
            pop_n_from_end(string,token_mapping.1.len());
            Some(token_mapping)
        }, None => None,
    })
}

fn pop_n_from_end(string: &mut String, n: usize) -> std::string::Drain<'_> {
    string.drain(string.len()-n..string.len())
}


fn string_ends_with<T: Token, L: Lexer<T>>(lexer: &L, string: &String) -> Result<Option<TokenMapping<T>>,()> {
    let ends_with = get_tokens_and_strings(lexer).iter()
        .filter_map(|TokenMapping(c, s)| -> Option<TokenMapping<T>> 
            { if string.ends_with(s) { Some(TokenMapping(c.clone(),s.clone())) } else { None } }
        ).collect::<Vec<TokenMapping<T>>>();
    // If there are multiple plausible token strings that this string could start with
    if ends_with.len() > 1 { return Err(()) }
    Ok(ends_with.get(0).cloned())
}

fn get_tokens<T: Token, L: Lexer<T>>(_: &L) -> Vec<T> { all::<T>().collect::<Vec<T>>() }
fn get_tokens_and_strings<T: Token, L: Lexer<T>>(lexer: &L) -> Vec<TokenMapping<T>> { 
    get_tokens(lexer)
        .iter()
        .map(|c| -> TokenMapping<T> { TokenMapping(c.clone(), lexer.string_from_token(c).clone()) })
        .collect()
}
