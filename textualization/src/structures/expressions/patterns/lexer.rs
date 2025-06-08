use enum_iterator::Sequence;

use crate::helpers::lexing::{Lexer, Token};

#[derive(Sequence, Clone, Copy, Debug)]
pub enum ExprPatternToken { VariableIndicator, VariableEnumerator }
impl Token for ExprPatternToken {}

#[derive(Clone,Debug,PartialEq,Eq)]
pub struct ExprPatternLexer {
    escape_string: String,
    variable_indicator: String,
    variable_enumerator: String,
}
impl ExprPatternLexer {
    pub fn new(escape_string: String, variable_indicator: String, variable_enumerator: String) -> Self 
        { Self { escape_string, variable_indicator, variable_enumerator } }
}
impl Lexer<ExprPatternToken> for ExprPatternLexer {
    fn string_from_token(&self, token: &ExprPatternToken) -> &String { match token {
        ExprPatternToken::VariableIndicator => &self.variable_indicator,
        ExprPatternToken::VariableEnumerator => &self.variable_enumerator,
    }}
    
    fn escape_string(&self) -> &String { &self.escape_string }
}
impl Default for ExprPatternLexer {
    fn default() -> Self { Self {
        escape_string: "\\".to_string(),
        variable_indicator: "#".to_string(),
        variable_enumerator: "..".to_string(),
    }}
}
