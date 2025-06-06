use enum_iterator::Sequence;
use vec::{VecToken, VecLexer};

use crate::helpers::{lexing::{Token, Lexer}, parsing::{ExprPatternToken, ExprPatternLexer}};

pub mod atom;
pub mod vec;
pub mod expressions;

#[derive(Sequence, Clone, Copy)]
pub enum TblStringifierToken { Vec(VecToken), Pattern (ExprPatternToken) }
impl Token for TblStringifierToken {}

#[derive(Clone)]
pub struct TblStringifierLexer{
    escape_string: String,
    vec_lexer: VecLexer,
    pattern_lexer: ExprPatternLexer,
}
impl TblStringifierLexer {
    pub fn new(escape_string: String, vec_lexer: VecLexer, pattern_lexer: ExprPatternLexer) -> Self
        { Self { escape_string, vec_lexer, pattern_lexer } }
}
impl Lexer<TblStringifierToken> for TblStringifierLexer {
    fn string_from_token(&self, token: &TblStringifierToken) -> &String { match token {
        TblStringifierToken::Vec(vec_token) => self.vec_lexer.string_from_token(vec_token),
        TblStringifierToken::Pattern(pattern_token) => self.pattern_lexer.string_from_token(pattern_token),
    }}
    
    fn escape_string(&self) -> &String { &self.escape_string }
}
impl Default for TblStringifierLexer {
    fn default() -> Self { Self {     
        escape_string: "\\".to_string(),
        vec_lexer: VecLexer::default(),
        pattern_lexer: ExprPatternLexer::default()
    }}
}
