use either::Either;
use enum_iterator::{all, Sequence};

use crate::{Detextualize, Textualizer, Textualize};

/// This trait allows an enum to be used as a Token for an implementer of the Lexer trait
pub trait Token: Sequence + Clone {}

#[derive(Clone)]
pub struct TokenMapping<T: Token>(pub T,pub String);

#[derive(Clone,PartialEq,Eq,Debug)]
pub struct TokenSequence<T: Token>(pub Vec<Either<T,String>>);

pub trait Lexer<T>: Clone + Send + Sync {
    fn escape_string(&self) -> &String;
    fn string_from_token(&self, token: &T) -> &String;
}

impl <T: Token, L: Lexer<T>> Textualizer<TokenSequence<T>> for L {}
impl <T: Token, L: Lexer<T>> Textualize<TokenSequence<T>> for L {
    fn textualize(&self, tokens: &TokenSequence<T>) -> Result<String,()> {
        let mut string = "".to_string();
        for s in &tokens.0 { match s {
            Either::Left(c) => string += self.string_from_token(&c),
            Either::Right(s) => string += s.as_str(),
        }}
        Ok(string)
    }
}
impl <T: Token, L: Lexer<T>> Detextualize<TokenSequence<T>> for L {
    fn detextualize(&self, string: &String) -> Result<TokenSequence<T>,()> {
        let mut tokens: Vec<Either<T, String>> = Vec::new();
        let mut escaping = false;
        let mut current_string = "".to_string();
        for char in string.chars() {
            current_string.push(char);
            if !escaping {
                if let Ok(_) = pop_escape_from_string_end(self,&mut current_string) { 
                    escaping = true;
                } else if let Some(token_mapping) = pop_token_from_string_end(self, &mut current_string)? {
                    if current_string.len() > 0 { tokens.push(Either::Right(current_string)) }
                    tokens.push(Either::Left(token_mapping.0));
                    current_string = "".to_owned();
                }
            } else { escaping = false; }
        }
        if current_string.len() > 0 { tokens.push(Either::Right(current_string)) }
        Ok(TokenSequence(tokens))
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

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use enum_iterator::Sequence;

    use super::*;

    #[derive(Sequence,Clone,PartialEq,Eq,Hash,Debug)]
    enum TestToken { A, BB }
    impl Token for TestToken {}

    #[derive(Clone)]
    struct TestLexer {
        escape: String,
        mapping: HashMap<TestToken,String>
    }
    impl Lexer<TestToken> for TestLexer {
        fn escape_string(&self) -> &String { &self.escape }
    
        fn string_from_token(&self, token: &TestToken) -> &String { 
            self.mapping.get(token).unwrap()
        }
    }
    impl Default for TestLexer {
        fn default() -> Self {
            let mapping = HashMap::from_iter(vec![
                (TestToken::A, "A".to_string()),
                (TestToken::BB, "BB".to_string()),
            ]);
            Self { escape: "\\".to_string(), mapping }
        }
    }

    fn pre_textualize_test(string: &str, tokens: Vec<Either<TestToken,&str>>) -> (Result<String,()>,String) {
        let lexer = TestLexer::default();
        let tokens = tokens.iter()
            .map(|obj| -> Either<TestToken,String> { match obj {
                Either::Left(token) => Either::Left(token.clone()),
                Either::Right(string) => Either::Right(string.to_string()),
            }}).collect();
        let sequence = TokenSequence(tokens);
        (lexer.textualize(&sequence), string.to_string())
    }

    fn pre_detextualize_test(string: &str, tokens: Vec<Either<TestToken,&str>>) -> (Result<TokenSequence<TestToken>,()>,TokenSequence<TestToken>) {
        let lexer = TestLexer::default();
        let tokens = tokens.iter()
            .map(|obj| -> Either<TestToken,String> { match obj {
                Either::Left(token) => Either::Left(token.clone()),
                Either::Right(string) => Either::Right(string.to_string()),
            }}).collect();
        let sequence = TokenSequence(tokens);
        (lexer.detextualize(&string.to_string()), sequence)
    }

    #[test]
    fn test_textualize_with_single_character_token() {
        let tokens = vec![Either::Left(TestToken::A)];
        let (textualized, check) = pre_textualize_test("A", tokens);
        assert_eq!(textualized, Ok(check));
    }

    #[test]
    fn test_detextualize_with_single_character_token() {
        let tokens = vec![Either::Left(TestToken::A)];
        let (detextualized, check) = pre_detextualize_test("A", tokens);
        assert_eq!(detextualized, Ok(check));
    }

    #[test]
    fn test_textualize_with_multi_character_token() {
        let tokens = vec![Either::Left(TestToken::BB)];
        let (textualized, check) = pre_textualize_test("BB", tokens);
        assert_eq!(textualized, Ok(check));
    }

    #[test]
    fn test_detextualize_with_multi_character_token() {
        let tokens = vec![Either::Left(TestToken::BB)];
        let (detextualized, check) = pre_detextualize_test("BB", tokens);
        assert_eq!(detextualized, Ok(check));
    }

    #[test]
    fn test_textualize_with_multiple_tokens() {
        let tokens = vec![Either::Left(TestToken::BB), Either::Left(TestToken::A), Either::Left(TestToken::A), Either::Left(TestToken::BB)];
        let (textualized, check) = pre_textualize_test("BBAABB", tokens);
        assert_eq!(textualized, Ok(check));
    }

    #[test]
    fn test_detextualize_with_multiple_tokens() {
        let tokens = vec![Either::Left(TestToken::BB), Either::Left(TestToken::A), Either::Left(TestToken::A), Either::Left(TestToken::BB)];
        let (detextualized, check) = pre_detextualize_test("BBAABB", tokens);
        assert_eq!(detextualized, Ok(check));
    }

    #[test]
    fn test_textualize_with_strings() {
        let tokens = vec![Either::Right("Sasquatch "), Either::Left(TestToken::BB), Either::Right("B"), Either::Left(TestToken::A), Either::Right("Firehose")];
        let (textualized, check) = pre_textualize_test("Sasquatch BBBAFirehose", tokens);
        assert_eq!(textualized, Ok(check));
    }

    #[test]
    fn test_detextualize_with_strings() {
        let tokens = vec![Either::Right("Sasquatch "), Either::Left(TestToken::BB), Either::Right("B"), Either::Left(TestToken::A), Either::Right("Firehose")];
        let (detextualized, check) = pre_detextualize_test("Sasquatch BBBAFirehose", tokens);
        assert_eq!(detextualized, Ok(check));
    }

    #[test]
    fn test_textualize_with_escapes() {
        let tokens = vec![Either::Right("Aardvark"), Either::Left(TestToken::BB)];
        let (textualized, check) = pre_textualize_test("\\AardvarkBB", tokens);
        assert_eq!(textualized, Ok(check));
    }

    #[test]
    fn test_detextualize_with_escapes() {
        let tokens = vec![Either::Right("Aardvark"), Either::Left(TestToken::BB)];
        let (detextualized, check) = pre_detextualize_test("\\AardvarkBB", tokens);
        assert_eq!(detextualized, Ok(check));
    }
}
