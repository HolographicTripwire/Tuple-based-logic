use enum_iterator::Sequence;

use crate::{helpers::lexing::{Token, Lexer}, Destringify, Stringifier, Stringify};

use super::{TblToken, TblLexer, };

#[derive(Default,Clone)]
pub struct VecParser(Box<TblLexer>);

impl Stringifier<Vec<String>> for VecParser {}
impl Stringify<Vec<String>> for VecParser {
    fn stringify(&self, strings: &Vec<String>) -> Result<String,()> {
        let opener = self.0.string_from_token(&TblToken::Vec(VecToken::Opener));
        let delimiter = self.0.string_from_token(&TblToken::Vec(VecToken::Delimiter));
        let closer = self.0.string_from_token(&TblToken::Vec(VecToken::Closer));
        Ok(opener.clone() + &strings.join(delimiter) + closer)
    }
}
impl Destringify<Vec<String>> for VecParser {
    fn destringify(&self, string: &String) -> Result<Vec<String>,()> {
        // Get token strings
        let vec_lexer = &self.0.vec_lexer;
        let escape_character = &self.0.escape_string;

        // Strip the opener and closer and get the inner string
        let inner =  {
            let mut s = string.as_str();
            if s.starts_with(&vec_lexer.opener) { s = &s[vec_lexer.opener.len()..s.len()]; }
            else { return Err(()) }
            if s.ends_with(&vec_lexer.closer) { s = &s[0..s.len()-vec_lexer.closer.len()]; }
            else { return Err(()) }
            s.to_string()
        };
        // Break into substrings
        let mut substrings = Vec::new(); // The substrings that we will return from this function
        let mut current_substring = "".to_string(); // The current substring, which will be added to substrings when we reach a delimiter
        let mut nesting_level = 0; // How deep into parentheses we are (e.g. if we had just read "((a,b),(c" we would be at level 2)
        let mut escaping = false; // If an escape character has just preceded the one we're at currently
        for char in inner.chars() {
            current_substring.push(char);
            // Handle delimiters. For example, if the delimiter is a comma (',') we will start a new substring on every comma
            if current_substring.ends_with(&vec_lexer.delimiter) && nesting_level == 0 && !escaping {
                current_substring = remove_from_end(current_substring, &vec_lexer.delimiter).unwrap();
                substrings.push(current_substring);
                current_substring = "".to_string();
            // Contain any nested vecs
            } else if current_substring.ends_with(&vec_lexer.opener) && !escaping {
                nesting_level += 1;
            } else if current_substring.ends_with(&vec_lexer.closer) && !escaping {
                nesting_level -= 1;
                // Parentheses level cannot be allowed to go below 0
                if nesting_level < 0 { return Err(()) }
            // Use backslash (\) as an escape character
            } else if current_substring.ends_with(escape_character) && !escaping {
                current_substring = remove_from_end(current_substring, escape_character).unwrap();
                escaping = true;
            // All non-token characters can 
            } else {
                escaping = false;
            }
        }
        // Push final substring, which will not be delimited like the others
        substrings.push(current_substring);
        // Then return all the substrings (each of which will be converted to a Expression later)
        Ok(substrings)
    }
}

fn remove_from_end(s1: String, s2: &String) -> Result<String,()> {
    if s1.ends_with(s2) { Ok(s1[0..s1.len()-s2.len()].to_string()) }
    else { Err(()) }
}

#[derive(Sequence, Clone, Copy, Debug)]
pub enum VecToken { Opener, Closer, Delimiter }
impl Token for VecToken {}

#[derive(Clone)]
pub struct VecLexer {
    escape_string: String,
    opener: String,
    closer: String,
    delimiter: String,
}
impl VecLexer {
    pub fn new(escape_string: String, opener: String, closer: String, delimiter: String) -> Self
        { Self { escape_string, opener, closer, delimiter } }
}
impl Lexer<VecToken> for VecLexer {
    fn string_from_token(&self, token: &VecToken) -> &String { match token {
        VecToken::Opener => &self.opener,
        VecToken::Closer => &self.closer,
        VecToken::Delimiter => &self.delimiter,
    }}
    
    fn escape_string(&self) -> &String { &self.escape_string }
}
impl Default for VecLexer {
    fn default() -> Self { VecLexer::new(
        "\\".to_string(),
        "(".to_string(),
        ")".to_string(),
        ",".to_string(),
    )}
}
