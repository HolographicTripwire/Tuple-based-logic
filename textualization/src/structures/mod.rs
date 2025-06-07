use enum_iterator::Sequence;
use vec::{VecToken, VecLexer};

use crate::helpers::{lexing::{Token, Lexer}, parsing::{ExprPatternToken, ExprPatternLexer}};

pub mod atom;
pub mod vec;
pub mod expressions;

#[derive(Sequence, Clone, Copy)]
pub enum TblToken { Vec(VecToken), Pattern (ExprPatternToken) }
impl Token for TblToken {}

#[derive(Clone)]
pub struct TblLexer{
    escape_string: String,
    vec_lexer: VecLexer,
    pattern_lexer: ExprPatternLexer,
}
impl TblLexer {
    pub fn new(escape_string: String, vec_lexer: VecLexer, pattern_lexer: ExprPatternLexer) -> Self
        { Self { escape_string, vec_lexer, pattern_lexer } }
}
impl Lexer<TblToken> for TblLexer {
    fn string_from_token(&self, token: &TblToken) -> &String { match token {
        TblToken::Vec(vec_token) => self.vec_lexer.string_from_token(vec_token),
        TblToken::Pattern(pattern_token) => self.pattern_lexer.string_from_token(pattern_token),
    }}
    
    fn escape_string(&self) -> &String { &self.escape_string }
}
impl Default for TblLexer {
    fn default() -> Self { Self {     
        escape_string: "\\".to_string(),
        vec_lexer: VecLexer::default(),
        pattern_lexer: ExprPatternLexer::default()
    }}
}
